use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type rt_context;
    pub type sym_version;
    pub type _tccdbg;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn _tcc_error_noabort(fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn section_ptr_add(sec: *mut Section, size: Elf64_Addr) -> *mut libc::c_void;
    fn get_sym_attr(
        s1: *mut TCCState,
        index: libc::c_int,
        alloc: libc::c_int,
    ) -> *mut sym_attr;
    fn tcc_enter_state(s1: *mut TCCState);
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
pub const ALWAYS_GOTPLT_ENTRY: gotplt_entry = 3;
pub const AUTO_GOTPLT_ENTRY: gotplt_entry = 2;
pub const NO_GOTPLT_ENTRY: gotplt_entry = 0;
pub type gotplt_entry = libc::c_uint;
pub const BUILD_GOT_ONLY: gotplt_entry = 1;
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
unsafe extern "C" fn add64le(mut p: *mut libc::c_uchar, mut x: int64_t) {
    write64le(p, (read64le(p)).wrapping_add(x as uint64_t));
}
#[no_mangle]
pub unsafe extern "C" fn code_reloc(mut reloc_type: libc::c_int) -> libc::c_int {
    match reloc_type {
        258 | 257 | 261 | 264 | 266 | 268 | 269 | 275 | 277 | 311 | 312 | 299 | 286 | 285
        | 284 | 278 | 1025 | 1024 => return 0 as libc::c_int,
        282 | 283 | 1026 => return 1 as libc::c_int,
        _ => {}
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn gotplt_entry_type(mut reloc_type: libc::c_int) -> libc::c_int {
    match reloc_type {
        261 | 264 | 266 | 268 | 269 | 275 | 277 | 299 | 286 | 285 | 284 | 278 | 1025
        | 1026 | 1024 => return NO_GOTPLT_ENTRY as libc::c_int,
        258 | 257 | 282 | 283 => return AUTO_GOTPLT_ENTRY as libc::c_int,
        311 | 312 => return ALWAYS_GOTPLT_ENTRY as libc::c_int,
        _ => {}
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn create_plt_entry(
    mut s1: *mut TCCState,
    mut got_offset: libc::c_uint,
    mut attr: *mut sym_attr,
) -> libc::c_uint {
    let mut plt: *mut Section = (*s1).plt;
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut plt_offset: libc::c_uint = 0;
    if (*plt).data_offset == 0 as libc::c_int as libc::c_ulong {
        section_ptr_add(plt, 32 as libc::c_int as Elf64_Addr);
    }
    plt_offset = (*plt).data_offset as libc::c_uint;
    p = section_ptr_add(plt, 16 as libc::c_int as Elf64_Addr) as *mut uint8_t;
    write32le(p, got_offset);
    write32le(
        p.offset(4 as libc::c_int as isize),
        (got_offset as uint64_t >> 32 as libc::c_int) as uint32_t,
    );
    return plt_offset;
}
#[no_mangle]
pub unsafe extern "C" fn relocate_plt(mut s1: *mut TCCState) {
    let mut p: *mut uint8_t = 0 as *mut uint8_t;
    let mut p_end: *mut uint8_t = 0 as *mut uint8_t;
    if ((*s1).plt).is_null() {
        return;
    }
    p = (*(*s1).plt).data;
    p_end = p.offset((*(*s1).plt).data_offset as isize);
    if p < p_end {
        let mut plt: uint64_t = (*(*s1).plt).sh_addr;
        let mut got: uint64_t = ((*(*s1).got).sh_addr)
            .wrapping_add(16 as libc::c_int as Elf64_Addr);
        let mut off: uint64_t = (got >> 12 as libc::c_int)
            .wrapping_sub(plt >> 12 as libc::c_int);
        if off
            .wrapping_add(
                ((1 as libc::c_int as uint32_t) << 20 as libc::c_int) as uint64_t,
            ) >> 21 as libc::c_int != 0
        {
            tcc_enter_state(s1);
            (Some(
                _tcc_error_noabort
                    as unsafe extern "C" fn(*const libc::c_char, ...) -> libc::c_int,
            ))
                .expect(
                    "non-null function pointer",
                )(
                b"Failed relocating PLT (off=0x%lx, got=0x%lx, plt=0x%lx)\0" as *const u8
                    as *const libc::c_char,
                off as libc::c_long,
                got as libc::c_long,
                plt as libc::c_long,
            );
        }
        write32le(p, 0xa9bf7bf0 as libc::c_uint);
        write32le(
            p.offset(4 as libc::c_int as isize),
            (0x90000010 as libc::c_uint as uint64_t
                | (off & 0x1ffffc as libc::c_int as uint64_t) << 3 as libc::c_int
                | (off & 3 as libc::c_int as uint64_t) << 29 as libc::c_int) as uint32_t,
        );
        write32le(
            p.offset(8 as libc::c_int as isize),
            (0xf9400211 as libc::c_uint as uint64_t
                | (got & 0xff8 as libc::c_int as uint64_t) << 7 as libc::c_int)
                as uint32_t,
        );
        write32le(
            p.offset(12 as libc::c_int as isize),
            (0x91000210 as libc::c_uint as uint64_t
                | (got & 0xfff as libc::c_int as uint64_t) << 10 as libc::c_int)
                as uint32_t,
        );
        write32le(p.offset(16 as libc::c_int as isize), 0xd61f0220 as libc::c_uint);
        write32le(p.offset(20 as libc::c_int as isize), 0xd503201f as libc::c_uint);
        write32le(p.offset(24 as libc::c_int as isize), 0xd503201f as libc::c_uint);
        write32le(p.offset(28 as libc::c_int as isize), 0xd503201f as libc::c_uint);
        p = p.offset(32 as libc::c_int as isize);
        got = (*(*s1).got).sh_addr;
        while p < p_end {
            let mut pc: uint64_t = plt
                .wrapping_add(
                    p.offset_from((*(*s1).plt).data) as libc::c_long as uint64_t,
                );
            let mut addr: uint64_t = got.wrapping_add(read64le(p));
            let mut off_0: uint64_t = (addr >> 12 as libc::c_int)
                .wrapping_sub(pc >> 12 as libc::c_int);
            if off_0
                .wrapping_add(
                    ((1 as libc::c_int as uint32_t) << 20 as libc::c_int) as uint64_t,
                ) >> 21 as libc::c_int != 0
            {
                tcc_enter_state(s1);
                (Some(
                    _tcc_error_noabort
                        as unsafe extern "C" fn(*const libc::c_char, ...) -> libc::c_int,
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"Failed relocating PLT (off=0x%lx, addr=0x%lx, pc=0x%lx)\0"
                        as *const u8 as *const libc::c_char,
                    off_0 as libc::c_long,
                    addr as libc::c_long,
                    pc as libc::c_long,
                );
            }
            write32le(
                p,
                (0x90000010 as libc::c_uint as uint64_t
                    | (off_0 & 0x1ffffc as libc::c_int as uint64_t) << 3 as libc::c_int
                    | (off_0 & 3 as libc::c_int as uint64_t) << 29 as libc::c_int)
                    as uint32_t,
            );
            write32le(
                p.offset(4 as libc::c_int as isize),
                (0xf9400211 as libc::c_uint as uint64_t
                    | (addr & 0xff8 as libc::c_int as uint64_t) << 7 as libc::c_int)
                    as uint32_t,
            );
            write32le(
                p.offset(8 as libc::c_int as isize),
                (0x91000210 as libc::c_uint as uint64_t
                    | (addr & 0xfff as libc::c_int as uint64_t) << 10 as libc::c_int)
                    as uint32_t,
            );
            write32le(p.offset(12 as libc::c_int as isize), 0xd61f0220 as libc::c_uint);
            p = p.offset(16 as libc::c_int as isize);
        }
    }
    if !((*(*s1).plt).reloc).is_null() {
        let mut rel: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
        p = (*(*s1).got).data;
        rel = ((*(*(*s1).plt).reloc).data as *mut Elf64_Rela)
            .offset(0 as libc::c_int as isize);
        while rel
            < ((*(*(*s1).plt).reloc).data)
                .offset((*(*(*s1).plt).reloc).data_offset as isize) as *mut Elf64_Rela
        {
            write64le(p.offset((*rel).r_offset as isize), (*(*s1).plt).sh_addr);
            rel = rel.offset(1);
            rel;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn relocate(
    mut s1: *mut TCCState,
    mut rel: *mut Elf64_Rela,
    mut type_0: libc::c_int,
    mut ptr: *mut libc::c_uchar,
    mut addr: Elf64_Addr,
    mut val: Elf64_Addr,
) {
    let mut sym_index: libc::c_int = ((*rel).r_info >> 32 as libc::c_int) as libc::c_int;
    let mut esym_index: libc::c_int = 0;
    let mut current_block_65: u64;
    match type_0 {
        257 => {
            if (*s1).output_type & 4 as libc::c_int != 0 {
                esym_index = (*get_sym_attr(s1, sym_index, 0 as libc::c_int)).dyn_index;
                (*(*s1).qrel).r_offset = (*rel).r_offset;
                if esym_index != 0 {
                    (*(*s1).qrel)
                        .r_info = ((esym_index as Elf64_Xword) << 32 as libc::c_int)
                        .wrapping_add(257 as libc::c_int as Elf64_Xword);
                    (*(*s1).qrel).r_addend = (*rel).r_addend;
                    (*s1).qrel = ((*s1).qrel).offset(1);
                    (*s1).qrel;
                    current_block_65 = 2520131295878969859;
                } else {
                    (*(*s1).qrel)
                        .r_info = ((0 as libc::c_int as Elf64_Xword)
                        << 32 as libc::c_int)
                        .wrapping_add(1027 as libc::c_int as Elf64_Xword);
                    (*(*s1).qrel)
                        .r_addend = (read64le(ptr)).wrapping_add(val) as Elf64_Sxword;
                    (*s1).qrel = ((*s1).qrel).offset(1);
                    (*s1).qrel;
                    current_block_65 = 3640593987805443782;
                }
            } else {
                current_block_65 = 3640593987805443782;
            }
            match current_block_65 {
                2520131295878969859 => {}
                _ => {
                    add64le(ptr, val as int64_t);
                    return;
                }
            }
        }
        258 => {
            if (*s1).output_type & 4 as libc::c_int != 0 {
                (*(*s1).qrel).r_offset = (*rel).r_offset;
                (*(*s1).qrel)
                    .r_info = ((0 as libc::c_int as Elf64_Xword) << 32 as libc::c_int)
                    .wrapping_add(1027 as libc::c_int as Elf64_Xword);
                (*(*s1).qrel)
                    .r_addend = (read32le(ptr) as libc::c_int as Elf64_Addr)
                    .wrapping_add(val) as Elf64_Sxword;
                (*s1).qrel = ((*s1).qrel).offset(1);
                (*s1).qrel;
            }
            add32le(ptr, val as int32_t);
            return;
        }
        261 => {
            if (*s1).output_type == 4 as libc::c_int {
                esym_index = (*get_sym_attr(s1, sym_index, 0 as libc::c_int)).dyn_index;
                if esym_index != 0 {
                    (*(*s1).qrel).r_offset = (*rel).r_offset;
                    (*(*s1).qrel)
                        .r_info = ((esym_index as Elf64_Xword) << 32 as libc::c_int)
                        .wrapping_add(261 as libc::c_int as Elf64_Xword);
                    (*(*s1).qrel)
                        .r_addend = read32le(ptr) as libc::c_int as Elf64_Sxword
                        + (*rel).r_addend;
                    (*s1).qrel = ((*s1).qrel).offset(1);
                    (*s1).qrel;
                    current_block_65 = 2520131295878969859;
                } else {
                    current_block_65 = 14401909646449704462;
                }
            } else {
                current_block_65 = 14401909646449704462;
            }
            match current_block_65 {
                2520131295878969859 => {}
                _ => {
                    add32le(ptr, val.wrapping_sub(addr) as int32_t);
                    return;
                }
            }
        }
        264 => {
            write32le(
                ptr,
                ((read32le(ptr) & 0xffe0001f as libc::c_uint) as Elf64_Addr
                    | (val & 0xffff as libc::c_int as Elf64_Addr) << 5 as libc::c_int)
                    as uint32_t,
            );
            return;
        }
        266 => {
            write32le(
                ptr,
                ((read32le(ptr) & 0xffe0001f as libc::c_uint) as Elf64_Addr
                    | (val >> 16 as libc::c_int & 0xffff as libc::c_int as Elf64_Addr)
                        << 5 as libc::c_int) as uint32_t,
            );
            return;
        }
        268 => {
            write32le(
                ptr,
                ((read32le(ptr) & 0xffe0001f as libc::c_uint) as Elf64_Addr
                    | (val >> 32 as libc::c_int & 0xffff as libc::c_int as Elf64_Addr)
                        << 5 as libc::c_int) as uint32_t,
            );
            return;
        }
        269 => {
            write32le(
                ptr,
                ((read32le(ptr) & 0xffe0001f as libc::c_uint) as Elf64_Addr
                    | (val >> 48 as libc::c_int & 0xffff as libc::c_int as Elf64_Addr)
                        << 5 as libc::c_int) as uint32_t,
            );
            return;
        }
        275 => {
            let mut off: uint64_t = (val >> 12 as libc::c_int)
                .wrapping_sub(addr >> 12 as libc::c_int);
            if off.wrapping_add((1 as libc::c_int as uint64_t) << 20 as libc::c_int)
                >> 21 as libc::c_int != 0
            {
                tcc_enter_state(s1);
                (Some(
                    _tcc_error_noabort
                        as unsafe extern "C" fn(*const libc::c_char, ...) -> libc::c_int,
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"R_AARCH64_ADR_PREL_PG_HI21 relocation failed\0" as *const u8
                        as *const libc::c_char,
                );
            }
            write32le(
                ptr,
                ((read32le(ptr) & 0x9f00001f as libc::c_uint) as uint64_t
                    | (off & 0x1ffffc as libc::c_int as uint64_t) << 3 as libc::c_int
                    | (off & 3 as libc::c_int as uint64_t) << 29 as libc::c_int)
                    as uint32_t,
            );
            return;
        }
        277 | 278 => {
            write32le(
                ptr,
                ((read32le(ptr) & 0xffc003ff as libc::c_uint) as Elf64_Addr
                    | (val & 0xfff as libc::c_int as Elf64_Addr) << 10 as libc::c_int)
                    as uint32_t,
            );
            return;
        }
        284 => {
            write32le(
                ptr,
                ((read32le(ptr) & 0xffc003ff as libc::c_uint) as Elf64_Addr
                    | (val & 0xffe as libc::c_int as Elf64_Addr) << 9 as libc::c_int)
                    as uint32_t,
            );
            return;
        }
        285 => {
            write32le(
                ptr,
                ((read32le(ptr) & 0xffc003ff as libc::c_uint) as Elf64_Addr
                    | (val & 0xffc as libc::c_int as Elf64_Addr) << 8 as libc::c_int)
                    as uint32_t,
            );
            return;
        }
        286 => {
            write32le(
                ptr,
                ((read32le(ptr) & 0xffc003ff as libc::c_uint) as Elf64_Addr
                    | (val & 0xff8 as libc::c_int as Elf64_Addr) << 7 as libc::c_int)
                    as uint32_t,
            );
            return;
        }
        299 => {
            write32le(
                ptr,
                ((read32le(ptr) & 0xffc003ff as libc::c_uint) as Elf64_Addr
                    | (val & 0xff0 as libc::c_int as Elf64_Addr) << 6 as libc::c_int)
                    as uint32_t,
            );
            return;
        }
        282 | 283 => {
            if val
                .wrapping_sub(addr)
                .wrapping_add((1 as libc::c_int as uint64_t) << 27 as libc::c_int)
                & !(0xffffffc as libc::c_int as uint64_t) != 0
            {
                tcc_enter_state(s1);
                (Some(
                    _tcc_error_noabort
                        as unsafe extern "C" fn(*const libc::c_char, ...) -> libc::c_int,
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"R_AARCH64_(JUMP|CALL)26 relocation failed (val=%lx, addr=%lx)\0"
                        as *const u8 as *const libc::c_char,
                    val as libc::c_long,
                    addr as libc::c_long,
                );
            }
            write32le(
                ptr,
                ((0x14000000 as libc::c_int as uint32_t
                    | ((type_0 == 283 as libc::c_int) as libc::c_int as uint32_t)
                        << 31 as libc::c_int) as Elf64_Addr
                    | val.wrapping_sub(addr) >> 2 as libc::c_int
                        & 0x3ffffff as libc::c_int as Elf64_Addr) as uint32_t,
            );
            return;
        }
        311 => {
            let mut off_0: uint64_t = (((*(*s1).got).sh_addr)
                .wrapping_add(
                    (*get_sym_attr(s1, sym_index, 0 as libc::c_int)).got_offset
                        as Elf64_Addr,
                ) >> 12 as libc::c_int)
                .wrapping_sub(addr >> 12 as libc::c_int);
            if off_0.wrapping_add((1 as libc::c_int as uint64_t) << 20 as libc::c_int)
                >> 21 as libc::c_int != 0
            {
                tcc_enter_state(s1);
                (Some(
                    _tcc_error_noabort
                        as unsafe extern "C" fn(*const libc::c_char, ...) -> libc::c_int,
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"R_AARCH64_ADR_GOT_PAGE relocation failed\0" as *const u8
                        as *const libc::c_char,
                );
            }
            write32le(
                ptr,
                ((read32le(ptr) & 0x9f00001f as libc::c_uint) as uint64_t
                    | (off_0 & 0x1ffffc as libc::c_int as uint64_t) << 3 as libc::c_int
                    | (off_0 & 3 as libc::c_int as uint64_t) << 29 as libc::c_int)
                    as uint32_t,
            );
            return;
        }
        312 => {
            write32le(
                ptr,
                ((read32le(ptr) & 0xfff803ff as libc::c_uint) as Elf64_Addr
                    | (((*(*s1).got).sh_addr)
                        .wrapping_add(
                            (*get_sym_attr(s1, sym_index, 0 as libc::c_int)).got_offset
                                as Elf64_Addr,
                        ) & 0xff8 as libc::c_int as Elf64_Addr) << 7 as libc::c_int)
                    as uint32_t,
            );
            return;
        }
        1024 => return,
        1025 | 1026 => {
            write64le(ptr, val.wrapping_sub((*rel).r_addend as Elf64_Addr));
            return;
        }
        1027 => return,
        _ => {
            fprintf(
                stderr,
                b"FIXME: handle reloc type %x at %x [%p] to %x\n\0" as *const u8
                    as *const libc::c_char,
                type_0,
                addr as libc::c_uint,
                ptr,
                val as libc::c_uint,
            );
            return;
        }
    };
}
