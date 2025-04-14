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
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
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
pub const ALWAYS_GOTPLT_ENTRY: gotplt_entry = 3;
pub const BUILD_GOT_ONLY: gotplt_entry = 1;
pub const AUTO_GOTPLT_ENTRY: gotplt_entry = 2;
pub const NO_GOTPLT_ENTRY: gotplt_entry = 0;
pub type gotplt_entry = libc::c_uint;
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
#[no_mangle]
pub unsafe extern "C" fn code_reloc(mut reloc_type: libc::c_int) -> libc::c_int {
    match reloc_type {
        8 | 20 | 1 | 10 | 9 | 3 | 43 | 6 | 5 | 18 | 19 | 32 | 17 => {
            return 0 as libc::c_int;
        }
        21 | 2 | 4 | 7 => return 1 as libc::c_int,
        _ => {}
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn gotplt_entry_type(mut reloc_type: libc::c_int) -> libc::c_int {
    match reloc_type {
        8 | 20 | 6 | 7 | 5 => return NO_GOTPLT_ENTRY as libc::c_int,
        1 => return AUTO_GOTPLT_ENTRY as libc::c_int,
        21 | 2 => return AUTO_GOTPLT_ENTRY as libc::c_int,
        10 | 9 => return BUILD_GOT_ONLY as libc::c_int,
        3 | 43 | 4 | 18 | 19 | 32 | 17 => return ALWAYS_GOTPLT_ENTRY as libc::c_int,
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
    let mut modrm: libc::c_int = 0;
    let mut plt_offset: libc::c_uint = 0;
    let mut relofs: libc::c_uint = 0;
    if (*s1).output_type & 4 as libc::c_int != 0 {
        modrm = 0xa3 as libc::c_int;
    } else {
        modrm = 0x25 as libc::c_int;
    }
    if (*plt).data_offset == 0 as libc::c_int as libc::c_ulong {
        p = section_ptr_add(plt, 16 as libc::c_int as Elf64_Addr) as *mut uint8_t;
        *p.offset(0 as libc::c_int as isize) = 0xff as libc::c_int as uint8_t;
        *p.offset(1 as libc::c_int as isize) = (modrm + 0x10 as libc::c_int) as uint8_t;
        write32le(p.offset(2 as libc::c_int as isize), 8 as libc::c_int as uint32_t);
        *p.offset(6 as libc::c_int as isize) = 0xff as libc::c_int as uint8_t;
        *p.offset(7 as libc::c_int as isize) = modrm as uint8_t;
        write32le(
            p.offset(8 as libc::c_int as isize),
            (8 as libc::c_int * 2 as libc::c_int) as uint32_t,
        );
    }
    plt_offset = (*plt).data_offset as libc::c_uint;
    relofs = (if !((*(*s1).plt).reloc).is_null() {
        (*(*(*s1).plt).reloc).data_offset
    } else {
        0 as libc::c_int as libc::c_ulong
    }) as libc::c_uint;
    p = section_ptr_add(plt, 16 as libc::c_int as Elf64_Addr) as *mut uint8_t;
    *p.offset(0 as libc::c_int as isize) = 0xff as libc::c_int as uint8_t;
    *p.offset(1 as libc::c_int as isize) = modrm as uint8_t;
    write32le(p.offset(2 as libc::c_int as isize), got_offset);
    *p.offset(6 as libc::c_int as isize) = 0x68 as libc::c_int as uint8_t;
    write32le(
        p.offset(7 as libc::c_int as isize),
        (relofs as libc::c_ulong)
            .wrapping_sub(::core::mem::size_of::<Elf64_Rela>() as libc::c_ulong)
            as uint32_t,
    );
    *p.offset(11 as libc::c_int as isize) = 0xe9 as libc::c_int as uint8_t;
    write32le(
        p.offset(12 as libc::c_int as isize),
        ((*plt).data_offset).wrapping_neg() as uint32_t,
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
    if (*s1).output_type & 4 as libc::c_int == 0 && p < p_end {
        add32le(p.offset(2 as libc::c_int as isize), (*(*s1).got).sh_addr as int32_t);
        add32le(p.offset(8 as libc::c_int as isize), (*(*s1).got).sh_addr as int32_t);
        p = p.offset(16 as libc::c_int as isize);
        while p < p_end {
            add32le(
                p.offset(2 as libc::c_int as isize),
                (*(*s1).got).sh_addr as int32_t,
            );
            p = p.offset(16 as libc::c_int as isize);
        }
    }
    if !((*(*s1).plt).reloc).is_null() {
        let mut rel: *mut Elf64_Rela = 0 as *mut Elf64_Rela;
        let mut x: libc::c_int = ((*(*s1).plt).sh_addr)
            .wrapping_add(16 as libc::c_int as Elf64_Addr)
            .wrapping_add(6 as libc::c_int as Elf64_Addr) as libc::c_int;
        p = (*(*s1).got).data;
        rel = ((*(*(*s1).plt).reloc).data as *mut Elf64_Rela)
            .offset(0 as libc::c_int as isize);
        while rel
            < ((*(*(*s1).plt).reloc).data)
                .offset((*(*(*s1).plt).reloc).data_offset as isize) as *mut Elf64_Rela
        {
            write32le(p.offset((*rel).r_offset as isize), x as uint32_t);
            x += 16 as libc::c_int;
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
    let mut sym_index: libc::c_int = 0;
    let mut esym_index: libc::c_int = 0;
    sym_index = ((*rel).r_info >> 32 as libc::c_int) as libc::c_int;
    let mut current_block_69: u64;
    match type_0 {
        1 => {
            if (*s1).output_type & 4 as libc::c_int != 0 {
                esym_index = (*get_sym_attr(s1, sym_index, 0 as libc::c_int)).dyn_index;
                (*(*s1).qrel).r_offset = (*rel).r_offset;
                if esym_index != 0 {
                    (*(*s1).qrel)
                        .r_info = ((esym_index as Elf64_Xword) << 32 as libc::c_int)
                        .wrapping_add(1 as libc::c_int as Elf64_Xword);
                    (*s1).qrel = ((*s1).qrel).offset(1);
                    (*s1).qrel;
                    return;
                } else {
                    (*(*s1).qrel)
                        .r_info = ((0 as libc::c_int as Elf64_Xword)
                        << 32 as libc::c_int)
                        .wrapping_add(8 as libc::c_int as Elf64_Xword);
                    (*s1).qrel = ((*s1).qrel).offset(1);
                    (*s1).qrel;
                }
            }
            add32le(ptr, val as int32_t);
            return;
        }
        2 => {
            if (*s1).output_type == 4 as libc::c_int {
                esym_index = (*get_sym_attr(s1, sym_index, 0 as libc::c_int)).dyn_index;
                if esym_index != 0 {
                    (*(*s1).qrel).r_offset = (*rel).r_offset;
                    (*(*s1).qrel)
                        .r_info = ((esym_index as Elf64_Xword) << 32 as libc::c_int)
                        .wrapping_add(2 as libc::c_int as Elf64_Xword);
                    (*s1).qrel = ((*s1).qrel).offset(1);
                    (*s1).qrel;
                    return;
                }
            }
            add32le(ptr, val.wrapping_sub(addr) as int32_t);
            return;
        }
        4 => {
            add32le(ptr, val.wrapping_sub(addr) as int32_t);
            return;
        }
        6 | 7 => {
            write32le(ptr, val as uint32_t);
            return;
        }
        10 => {
            add32le(ptr, ((*(*s1).got).sh_addr).wrapping_sub(addr) as int32_t);
            return;
        }
        9 => {
            add32le(ptr, val.wrapping_sub((*(*s1).got).sh_addr) as int32_t);
            return;
        }
        3 | 43 => {
            add32le(
                ptr,
                (*get_sym_attr(s1, sym_index, 0 as libc::c_int)).got_offset as int32_t,
            );
            return;
        }
        20 => {
            if (*s1).output_format != 1 as libc::c_int {
                current_block_69 = 13473903116748164871;
            } else {
                current_block_69 = 11459959175219260272;
            }
        }
        21 => {
            if (*s1).output_format != 1 as libc::c_int {
                current_block_69 = 13473903116748164871;
            } else {
                write16le(
                    ptr,
                    (read16le(ptr) as Elf64_Addr).wrapping_add(val).wrapping_sub(addr)
                        as uint16_t,
                );
                return;
            }
        }
        8 => return,
        5 => return,
        18 => {
            static mut expect: [libc::c_uchar; 12] = [
                0x8d as libc::c_int as libc::c_uchar,
                0x4 as libc::c_int as libc::c_uchar,
                0x1d as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0xe8 as libc::c_int as libc::c_uchar,
                0xfc as libc::c_int as libc::c_uchar,
                0xff as libc::c_int as libc::c_uchar,
                0xff as libc::c_int as libc::c_uchar,
                0xff as libc::c_int as libc::c_uchar,
            ];
            static mut replace: [libc::c_uchar; 12] = [
                0x65 as libc::c_int as libc::c_uchar,
                0xa1 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0x81 as libc::c_int as libc::c_uchar,
                0xe8 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
            ];
            if memcmp(
                ptr.offset(-(3 as libc::c_int as isize)) as *const libc::c_void,
                expect.as_ptr() as *const libc::c_void,
                ::core::mem::size_of::<[libc::c_uchar; 12]>() as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
                let mut sec: *mut Section = 0 as *mut Section;
                let mut x: int32_t = 0;
                memcpy(
                    ptr.offset(-(3 as libc::c_int as isize)) as *mut libc::c_void,
                    replace.as_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<[libc::c_uchar; 12]>() as libc::c_ulong,
                );
                (*rel.offset(1 as libc::c_int as isize))
                    .r_info = ((0 as libc::c_int as Elf64_Xword) << 32 as libc::c_int)
                    .wrapping_add(0 as libc::c_int as Elf64_Xword);
                sym = &mut *((*(*s1).c2rust_unnamed.symtab_section).data
                    as *mut Elf64_Sym)
                    .offset(sym_index as isize) as *mut Elf64_Sym;
                sec = *((*s1).sections).offset((*sym).st_shndx as isize);
                x = ((*sym).st_value)
                    .wrapping_sub((*sec).sh_addr)
                    .wrapping_sub((*sec).data_offset) as int32_t;
                add32le(ptr.offset(5 as libc::c_int as isize), -x);
            } else {
                tcc_enter_state(s1);
                (Some(
                    _tcc_error_noabort
                        as unsafe extern "C" fn(*const libc::c_char, ...) -> libc::c_int,
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"unexpected R_386_TLS_GD pattern\0" as *const u8
                        as *const libc::c_char,
                );
            }
            return;
        }
        19 => {
            static mut expect_0: [libc::c_uchar; 11] = [
                0x8d as libc::c_int as libc::c_uchar,
                0x83 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0xe8 as libc::c_int as libc::c_uchar,
                0xfc as libc::c_int as libc::c_uchar,
                0xff as libc::c_int as libc::c_uchar,
                0xff as libc::c_int as libc::c_uchar,
                0xff as libc::c_int as libc::c_uchar,
            ];
            static mut replace_0: [libc::c_uchar; 11] = [
                0x65 as libc::c_int as libc::c_uchar,
                0xa1 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
                0x90 as libc::c_int as libc::c_uchar,
                0x8d as libc::c_int as libc::c_uchar,
                0x74 as libc::c_int as libc::c_uchar,
                0x26 as libc::c_int as libc::c_uchar,
                0 as libc::c_int as libc::c_uchar,
            ];
            if memcmp(
                ptr.offset(-(2 as libc::c_int as isize)) as *const libc::c_void,
                expect_0.as_ptr() as *const libc::c_void,
                ::core::mem::size_of::<[libc::c_uchar; 11]>() as libc::c_ulong,
            ) == 0 as libc::c_int
            {
                memcpy(
                    ptr.offset(-(2 as libc::c_int as isize)) as *mut libc::c_void,
                    replace_0.as_ptr() as *const libc::c_void,
                    ::core::mem::size_of::<[libc::c_uchar; 11]>() as libc::c_ulong,
                );
                (*rel.offset(1 as libc::c_int as isize))
                    .r_info = ((0 as libc::c_int as Elf64_Xword) << 32 as libc::c_int)
                    .wrapping_add(0 as libc::c_int as Elf64_Xword);
            } else {
                tcc_enter_state(s1);
                (Some(
                    _tcc_error_noabort
                        as unsafe extern "C" fn(*const libc::c_char, ...) -> libc::c_int,
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"unexpected R_386_TLS_LDM pattern\0" as *const u8
                        as *const libc::c_char,
                );
            }
            return;
        }
        32 | 17 => {
            let mut sym_0: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
            let mut sec_0: *mut Section = 0 as *mut Section;
            let mut x_0: int32_t = 0;
            sym_0 = &mut *((*(*s1).c2rust_unnamed.symtab_section).data as *mut Elf64_Sym)
                .offset(sym_index as isize) as *mut Elf64_Sym;
            sec_0 = *((*s1).sections).offset((*sym_0).st_shndx as isize);
            x_0 = val.wrapping_sub((*sec_0).sh_addr).wrapping_sub((*sec_0).data_offset)
                as int32_t;
            add32le(ptr, x_0);
            return;
        }
        0 => return,
        _ => {
            fprintf(
                stderr,
                b"FIXME: handle reloc type %d at %x [%p] to %x\n\0" as *const u8
                    as *const libc::c_char,
                type_0,
                addr as libc::c_uint,
                ptr,
                val as libc::c_uint,
            );
            return;
        }
    }
    match current_block_69 {
        13473903116748164871 => {
            tcc_enter_state(s1);
            (Some(
                _tcc_error_noabort
                    as unsafe extern "C" fn(*const libc::c_char, ...) -> libc::c_int,
            ))
                .expect(
                    "non-null function pointer",
                )(
                b"can only produce 16-bit binary files\0" as *const u8
                    as *const libc::c_char,
            );
        }
        _ => {}
    }
    write16le(ptr, (read16le(ptr) as Elf64_Addr).wrapping_add(val) as uint16_t);
}
