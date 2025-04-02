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
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn _tcc_error_noabort(fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn section_ptr_add(sec: *mut Section, size: Elf64_Addr) -> *mut libc::c_void;
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
#[no_mangle]
pub unsafe extern "C" fn code_reloc(mut reloc_type: libc::c_int) -> libc::c_int {
    match reloc_type {
        44 | 43 | 48 | 47 | 2 | 3 | 25 | 24 | 26 | 96 | 20 | 21 | 0 | 38 | 46 | 45 => {
            return 0 as libc::c_int;
        }
        1 | 28 | 29 | 27 | 10 | 30 | 42 | 40 | 22 => return 1 as libc::c_int,
        _ => {}
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn gotplt_entry_type(mut reloc_type: libc::c_int) -> libc::c_int {
    match reloc_type {
        0 | 20 | 21 | 22 => return NO_GOTPLT_ENTRY as libc::c_int,
        1 | 28 | 29 | 27 | 10 | 30 | 44 | 43 | 48 | 47 | 42 | 2 | 3 | 40 | 38 | 46
        | 45 => return AUTO_GOTPLT_ENTRY as libc::c_int,
        25 | 24 => return BUILD_GOT_ONLY as libc::c_int,
        26 | 96 => return ALWAYS_GOTPLT_ENTRY as libc::c_int,
        _ => {}
    }
    return -(1 as libc::c_int);
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
        let mut x: libc::c_int = ((*(*s1).got).sh_addr)
            .wrapping_sub((*(*s1).plt).sh_addr)
            .wrapping_sub(12 as libc::c_int as Elf64_Addr) as libc::c_int;
        write32le(
            ((*(*s1).plt).data).offset(16 as libc::c_int as isize),
            (x - 4 as libc::c_int) as uint32_t,
        );
        p = p.offset(20 as libc::c_int as isize);
        while p < p_end {
            let mut off: libc::c_uint = ((x as uint32_t)
                .wrapping_add(read32le(p.offset(4 as libc::c_int as isize)))
                as libc::c_long + ((*(*s1).plt).data).offset_from(p) as libc::c_long
                + 4 as libc::c_int as libc::c_long) as libc::c_uint;
            if read32le(p) == 0x46c04778 as libc::c_int as uint32_t {
                p = p.offset(4 as libc::c_int as isize);
            }
            write32le(
                p,
                0xe28fc200 as libc::c_uint
                    | off >> 28 as libc::c_int & 0xf as libc::c_int as libc::c_uint,
            );
            write32le(
                p.offset(4 as libc::c_int as isize),
                0xe28cc600 as libc::c_uint
                    | off >> 20 as libc::c_int & 0xff as libc::c_int as libc::c_uint,
            );
            write32le(
                p.offset(8 as libc::c_int as isize),
                0xe28cca00 as libc::c_uint
                    | off >> 12 as libc::c_int & 0xff as libc::c_int as libc::c_uint,
            );
            write32le(
                p.offset(12 as libc::c_int as isize),
                0xe5bcf000 as libc::c_uint | off & 0xfff as libc::c_int as libc::c_uint,
            );
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
            write32le(
                p.offset((*rel).r_offset as isize),
                (*(*s1).plt).sh_addr as uint32_t,
            );
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
    let mut sym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    let mut sym_index: libc::c_int = 0;
    let mut esym_index: libc::c_int = 0;
    sym_index = ((*rel).r_info >> 32 as libc::c_int) as libc::c_int;
    sym = &mut *((*(*s1).c2rust_unnamed.symtab_section).data as *mut Elf64_Sym)
        .offset(sym_index as isize) as *mut Elf64_Sym;
    match type_0 {
        1 | 28 | 29 | 27 => {
            let mut x: libc::c_int = 0;
            let mut is_thumb: libc::c_int = 0;
            let mut is_call: libc::c_int = 0;
            let mut h: libc::c_int = 0;
            let mut blx_avail: libc::c_int = 0;
            let mut is_bl: libc::c_int = 0;
            let mut th_ko: libc::c_int = 0;
            x = *(ptr as *mut libc::c_int) & 0xffffff as libc::c_int;
            let ref mut fresh0 = *(ptr as *mut libc::c_int);
            *fresh0 = (*fresh0 as libc::c_uint & 0xff000000 as libc::c_uint)
                as libc::c_int;
            if x & 0x800000 as libc::c_int != 0 {
                x -= 0x1000000 as libc::c_int;
            }
            x <<= 2 as libc::c_int;
            is_thumb = (val & 1 as libc::c_int as Elf64_Addr) as libc::c_int;
            is_bl = (*(ptr as *mut libc::c_uint) >> 24 as libc::c_int
                == 0xeb as libc::c_int as libc::c_uint) as libc::c_int;
            is_call = (type_0 == 28 as libc::c_int
                || type_0 == 1 as libc::c_int && is_bl != 0) as libc::c_int;
            x = (x as Elf64_Addr).wrapping_add(val.wrapping_sub(addr)) as libc::c_int
                as libc::c_int;
            h = x & 2 as libc::c_int;
            th_ko = (x & 3 as libc::c_int != 0 && (blx_avail == 0 || is_call == 0))
                as libc::c_int;
            if th_ko != 0 || x >= 0x2000000 as libc::c_int
                || x < -(0x2000000 as libc::c_int)
            {
                tcc_enter_state(s1);
                (Some(
                    _tcc_error_noabort
                        as unsafe extern "C" fn(*const libc::c_char, ...) -> libc::c_int,
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"can't relocate value at %x,%d\0" as *const u8
                        as *const libc::c_char,
                    addr,
                    type_0,
                );
            }
            x >>= 2 as libc::c_int;
            x &= 0xffffff as libc::c_int;
            if is_thumb != 0 {
                x |= h << 24 as libc::c_int;
                *(ptr as *mut libc::c_int) = (0xfa as libc::c_int) << 24 as libc::c_int;
            }
            *(ptr as *mut libc::c_int) |= x;
            return;
        }
        10 | 30 => {
            let mut x_0: libc::c_int = 0;
            let mut hi: libc::c_int = 0;
            let mut lo: libc::c_int = 0;
            let mut s: libc::c_int = 0;
            let mut j1: libc::c_int = 0;
            let mut j2: libc::c_int = 0;
            let mut i1: libc::c_int = 0;
            let mut i2: libc::c_int = 0;
            let mut imm10: libc::c_int = 0;
            let mut imm11: libc::c_int = 0;
            let mut to_thumb: libc::c_int = 0;
            let mut is_call_0: libc::c_int = 0;
            let mut to_plt: libc::c_int = 0;
            let mut blx_bit: libc::c_int = (1 as libc::c_int) << 12 as libc::c_int;
            let mut plt: *mut Section = 0 as *mut Section;
            if (*sym).st_shndx as libc::c_int == 0 as libc::c_int
                && (*sym).st_info as libc::c_int >> 4 as libc::c_int == 2 as libc::c_int
            {
                return;
            }
            hi = *(ptr as *mut uint16_t) as libc::c_int;
            lo = *(ptr.offset(2 as libc::c_int as isize) as *mut uint16_t)
                as libc::c_int;
            s = hi >> 10 as libc::c_int & 1 as libc::c_int;
            j1 = lo >> 13 as libc::c_int & 1 as libc::c_int;
            j2 = lo >> 11 as libc::c_int & 1 as libc::c_int;
            i1 = j1 ^ s ^ 1 as libc::c_int;
            i2 = j2 ^ s ^ 1 as libc::c_int;
            imm10 = hi & 0x3ff as libc::c_int;
            imm11 = lo & 0x7ff as libc::c_int;
            x_0 = s << 24 as libc::c_int | i1 << 23 as libc::c_int
                | i2 << 22 as libc::c_int | imm10 << 12 as libc::c_int
                | imm11 << 1 as libc::c_int;
            if x_0 & 0x1000000 as libc::c_int != 0 {
                x_0 -= 0x2000000 as libc::c_int;
            }
            to_thumb = (val & 1 as libc::c_int as Elf64_Addr) as libc::c_int;
            plt = (*s1).plt;
            to_plt = (val >= (*plt).sh_addr
                && val < ((*plt).sh_addr).wrapping_add((*plt).data_offset))
                as libc::c_int;
            is_call_0 = (type_0 == 10 as libc::c_int) as libc::c_int;
            if to_thumb == 0 && to_plt == 0 && is_call_0 == 0 {
                let mut index: libc::c_int = 0;
                let mut p: *mut uint8_t = 0 as *mut uint8_t;
                let mut name: *mut libc::c_char = 0 as *mut libc::c_char;
                let mut buf: [libc::c_char; 1024] = [0; 1024];
                let mut text: *mut Section = 0 as *mut Section;
                name = ((*(*(*s1).c2rust_unnamed.symtab_section).link).data
                    as *mut libc::c_char)
                    .offset((*sym).st_name as isize);
                text = *((*s1).sections).offset((*sym).st_shndx as isize);
                snprintf(
                    buf.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 1024]>() as libc::c_ulong,
                    b"%s_from_thumb\0" as *const u8 as *const libc::c_char,
                    name,
                );
                index = put_elf_sym(
                    (*s1).c2rust_unnamed.symtab_section,
                    ((*text).data_offset)
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                    (*sym).st_size,
                    (*sym).st_info as libc::c_int,
                    0 as libc::c_int,
                    (*sym).st_shndx as libc::c_int,
                    buf.as_mut_ptr(),
                );
                to_thumb = 1 as libc::c_int;
                val = ((*text).data_offset)
                    .wrapping_add(1 as libc::c_int as libc::c_ulong);
                (*rel)
                    .r_info = ((index as Elf64_Xword) << 32 as libc::c_int)
                    .wrapping_add(type_0 as Elf64_Xword);
                put_elf_reloc(
                    (*s1).c2rust_unnamed.symtab_section,
                    text,
                    ((*text).data_offset)
                        .wrapping_add(4 as libc::c_int as libc::c_ulong),
                    29 as libc::c_int,
                    sym_index,
                );
                p = section_ptr_add(text, 8 as libc::c_int as Elf64_Addr)
                    as *mut uint8_t;
                write32le(p, 0x4778 as libc::c_int as uint32_t);
                write32le(
                    p.offset(2 as libc::c_int as isize),
                    0x46c0 as libc::c_int as uint32_t,
                );
                write32le(
                    p.offset(4 as libc::c_int as isize),
                    0xeafffffe as libc::c_uint,
                );
            }
            x_0 = (x_0 as Elf64_Addr).wrapping_add(val.wrapping_sub(addr)) as libc::c_int
                as libc::c_int;
            if to_thumb == 0 && is_call_0 != 0 {
                blx_bit = 0 as libc::c_int;
                x_0 = x_0 + 3 as libc::c_int & -(4 as libc::c_int);
            }
            if to_thumb == 0 || x_0 >= 0x1000000 as libc::c_int
                || x_0 < -(0x1000000 as libc::c_int)
            {
                if to_thumb != 0 || val & 2 as libc::c_int as Elf64_Addr != 0
                    || is_call_0 == 0 && to_plt == 0
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
                        b"can't relocate value at %x,%d\0" as *const u8
                            as *const libc::c_char,
                        addr,
                        type_0,
                    );
                }
            }
            s = x_0 >> 24 as libc::c_int & 1 as libc::c_int;
            i1 = x_0 >> 23 as libc::c_int & 1 as libc::c_int;
            i2 = x_0 >> 22 as libc::c_int & 1 as libc::c_int;
            j1 = s ^ (i1 ^ 1 as libc::c_int);
            j2 = s ^ (i2 ^ 1 as libc::c_int);
            imm10 = x_0 >> 12 as libc::c_int & 0x3ff as libc::c_int;
            imm11 = x_0 >> 1 as libc::c_int & 0x7ff as libc::c_int;
            *(ptr
                as *mut uint16_t) = (hi & 0xf800 as libc::c_int | s << 10 as libc::c_int
                | imm10) as uint16_t;
            *(ptr.offset(2 as libc::c_int as isize)
                as *mut uint16_t) = (lo & 0xc000 as libc::c_int | j1 << 13 as libc::c_int
                | blx_bit | j2 << 11 as libc::c_int | imm11) as uint16_t;
            return;
        }
        44 | 43 => {
            let mut x_1: libc::c_int = 0;
            let mut imm4: libc::c_int = 0;
            let mut imm12: libc::c_int = 0;
            if type_0 == 44 as libc::c_int {
                val >>= 16 as libc::c_int;
            }
            imm12 = (val & 0xfff as libc::c_int as Elf64_Addr) as libc::c_int;
            imm4 = (val >> 12 as libc::c_int & 0xf as libc::c_int as Elf64_Addr)
                as libc::c_int;
            x_1 = imm4 << 16 as libc::c_int | imm12;
            if type_0 == 48 as libc::c_int {
                *(ptr as *mut libc::c_int) |= x_1;
            } else {
                *(ptr as *mut libc::c_int) += x_1;
            }
            return;
        }
        46 | 45 => {
            let mut insn: libc::c_int = *(ptr as *mut libc::c_int);
            let mut addend: libc::c_int = insn >> 4 as libc::c_int
                & 0xf000 as libc::c_int | insn & 0xfff as libc::c_int;
            addend = (addend ^ 0x8000 as libc::c_int) - 0x8000 as libc::c_int;
            val = val.wrapping_add((addend as Elf64_Addr).wrapping_sub(addr));
            if type_0 == 46 as libc::c_int {
                val >>= 16 as libc::c_int;
            }
            *(ptr
                as *mut libc::c_int) = ((insn as libc::c_uint
                & 0xfff0f000 as libc::c_uint) as Elf64_Addr
                | (val & 0xf000 as libc::c_int as Elf64_Addr) << 4 as libc::c_int
                | val & 0xfff as libc::c_int as Elf64_Addr) as libc::c_int;
            return;
        }
        48 | 47 => {
            let mut x_2: libc::c_int = 0;
            let mut i: libc::c_int = 0;
            let mut imm4_0: libc::c_int = 0;
            let mut imm3: libc::c_int = 0;
            let mut imm8: libc::c_int = 0;
            if type_0 == 48 as libc::c_int {
                val >>= 16 as libc::c_int;
            }
            imm8 = (val & 0xff as libc::c_int as Elf64_Addr) as libc::c_int;
            imm3 = (val >> 8 as libc::c_int & 0x7 as libc::c_int as Elf64_Addr)
                as libc::c_int;
            i = (val >> 11 as libc::c_int & 1 as libc::c_int as Elf64_Addr)
                as libc::c_int;
            imm4_0 = (val >> 12 as libc::c_int & 0xf as libc::c_int as Elf64_Addr)
                as libc::c_int;
            x_2 = imm3 << 28 as libc::c_int | imm8 << 16 as libc::c_int
                | i << 10 as libc::c_int | imm4_0;
            if type_0 == 48 as libc::c_int {
                *(ptr as *mut libc::c_int) |= x_2;
            } else {
                *(ptr as *mut libc::c_int) += x_2;
            }
            return;
        }
        42 => {
            let mut x_3: libc::c_int = 0;
            x_3 = *(ptr as *mut libc::c_int) & 0x7fffffff as libc::c_int;
            let ref mut fresh1 = *(ptr as *mut libc::c_int);
            *fresh1 = (*fresh1 as libc::c_uint & 0x80000000 as libc::c_uint)
                as libc::c_int;
            x_3 = x_3 * 2 as libc::c_int / 2 as libc::c_int;
            x_3 = (x_3 as Elf64_Addr).wrapping_add(val.wrapping_sub(addr)) as libc::c_int
                as libc::c_int;
            if (x_3 ^ x_3 >> 1 as libc::c_int) & 0x40000000 as libc::c_int != 0 {
                tcc_enter_state(s1);
                (Some(
                    _tcc_error_noabort
                        as unsafe extern "C" fn(*const libc::c_char, ...) -> libc::c_int,
                ))
                    .expect(
                        "non-null function pointer",
                    )(
                    b"can't relocate value at %x,%d\0" as *const u8
                        as *const libc::c_char,
                    addr,
                    type_0,
                );
            }
            *(ptr as *mut libc::c_int) |= x_3 & 0x7fffffff as libc::c_int;
            return;
        }
        2 | 38 => {
            if (*s1).output_type & 4 as libc::c_int != 0 {
                esym_index = (*get_sym_attr(s1, sym_index, 0 as libc::c_int)).dyn_index;
                (*(*s1).qrel).r_offset = (*rel).r_offset;
                if esym_index != 0 {
                    (*(*s1).qrel)
                        .r_info = ((esym_index as Elf64_Xword) << 32 as libc::c_int)
                        .wrapping_add(2 as libc::c_int as Elf64_Xword);
                    (*s1).qrel = ((*s1).qrel).offset(1);
                    (*s1).qrel;
                    return;
                } else {
                    (*(*s1).qrel)
                        .r_info = ((0 as libc::c_int as Elf64_Xword)
                        << 32 as libc::c_int)
                        .wrapping_add(23 as libc::c_int as Elf64_Xword);
                    (*s1).qrel = ((*s1).qrel).offset(1);
                    (*s1).qrel;
                }
            }
            let ref mut fresh2 = *(ptr as *mut libc::c_int);
            *fresh2 = (*fresh2 as Elf64_Addr).wrapping_add(val) as libc::c_int
                as libc::c_int;
            return;
        }
        3 => {
            let ref mut fresh3 = *(ptr as *mut libc::c_int);
            *fresh3 = (*fresh3 as Elf64_Addr).wrapping_add(val.wrapping_sub(addr))
                as libc::c_int as libc::c_int;
            return;
        }
        25 => {
            let ref mut fresh4 = *(ptr as *mut libc::c_int);
            *fresh4 = (*fresh4 as Elf64_Addr)
                .wrapping_add(((*(*s1).got).sh_addr).wrapping_sub(addr)) as libc::c_int
                as libc::c_int;
            return;
        }
        24 => {
            let ref mut fresh5 = *(ptr as *mut libc::c_int);
            *fresh5 = (*fresh5 as Elf64_Addr)
                .wrapping_add(val.wrapping_sub((*(*s1).got).sh_addr)) as libc::c_int
                as libc::c_int;
            return;
        }
        26 => {
            let ref mut fresh6 = *(ptr as *mut libc::c_int);
            *fresh6 = (*fresh6 as libc::c_uint)
                .wrapping_add(
                    (*get_sym_attr(s1, sym_index, 0 as libc::c_int)).got_offset,
                ) as libc::c_int as libc::c_int;
            return;
        }
        96 => {
            let ref mut fresh7 = *(ptr as *mut libc::c_int);
            *fresh7 = (*fresh7 as Elf64_Addr)
                .wrapping_add(
                    ((*(*s1).got).sh_addr)
                        .wrapping_add(
                            (*get_sym_attr(s1, sym_index, 0 as libc::c_int)).got_offset
                                as Elf64_Addr,
                        )
                        .wrapping_sub(addr),
                ) as libc::c_int as libc::c_int;
            return;
        }
        20 => return,
        40 => {
            if 0xffffff0 as libc::c_int & *(ptr as *mut libc::c_int)
                == 0x12fff10 as libc::c_int
            {
                let ref mut fresh8 = *(ptr as *mut libc::c_int);
                *fresh8 = (*fresh8 as libc::c_uint
                    ^ (0xe12fff10 as libc::c_uint ^ 0xe1a0f000 as libc::c_uint))
                    as libc::c_int;
            }
            return;
        }
        21 | 22 => {
            *(ptr as *mut Elf64_Addr) = val;
            return;
        }
        0 => return,
        23 => return,
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
    };
}
