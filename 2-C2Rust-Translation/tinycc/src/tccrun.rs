use ::libc;
use ::c2rust_bitfields;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type sym_version;
    pub type _tccdbg;
    fn exit(_: libc::c_int) -> !;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn __errno_location() -> *mut libc::c_int;
    fn _setjmp(_: *mut __jmp_buf_tag) -> libc::c_int;
    fn longjmp(_: *mut __jmp_buf_tag, _: libc::c_int) -> !;
    static mut environ: *mut *mut libc::c_char;
    fn sysconf(__name: libc::c_int) -> libc::c_long;
    fn dlclose(__handle: *mut libc::c_void) -> libc::c_int;
    fn pstrcpy(
        buf: *mut libc::c_char,
        buf_size: size_t,
        s: *const libc::c_char,
    ) -> *mut libc::c_char;
    fn tcc_free(ptr: *mut libc::c_void);
    fn tcc_malloc(size: libc::c_ulong) -> *mut libc::c_void;
    fn tcc_realloc(ptr: *mut libc::c_void, size: libc::c_ulong) -> *mut libc::c_void;
    fn _tcc_error_noabort(fmt: *const libc::c_char, _: ...) -> libc::c_int;
    fn tcc_add_support(s1: *mut TCCState, filename: *const libc::c_char) -> libc::c_int;
    fn free_section(s: *mut Section);
    fn init_symtab(s: *mut Section);
    fn put_elf_sym(
        s: *mut Section,
        value: Elf64_Addr,
        size: libc::c_ulong,
        info: libc::c_int,
        other: libc::c_int,
        shndx: libc::c_int,
        name: *const libc::c_char,
    ) -> libc::c_int;
    fn resolve_common_syms(s1: *mut TCCState);
    fn relocate_syms(s1: *mut TCCState, symtab: *mut Section, do_resolve: libc::c_int);
    fn relocate_sections(s1: *mut TCCState);
    fn get_sym_addr(
        s: *mut TCCState,
        name: *const libc::c_char,
        err: libc::c_int,
        forc: libc::c_int,
    ) -> Elf64_Addr;
    fn tcc_add_runtime(s1: *mut TCCState);
    fn relocate_plt(s1: *mut TCCState);
    fn build_got_entries(s1: *mut TCCState, got_sym: libc::c_int);
    fn tcc_enter_state(s1: *mut TCCState);
    fn tcc_add_symbol(
        s: *mut TCCState,
        name: *const libc::c_char,
        val: *const libc::c_void,
    ) -> libc::c_int;
    fn tcc_get_symbol(s: *mut TCCState, name: *const libc::c_char) -> *mut libc::c_void;
    fn sem_init(
        __sem: *mut sem_t,
        __pshared: libc::c_int,
        __value: libc::c_uint,
    ) -> libc::c_int;
    fn sem_wait(__sem: *mut sem_t) -> libc::c_int;
    fn sem_post(__sem: *mut sem_t) -> libc::c_int;
    fn mprotect(
        __addr: *mut libc::c_void,
        __len: size_t,
        __prot: libc::c_int,
    ) -> libc::c_int;
    fn sigemptyset(__set: *mut sigset_t) -> libc::c_int;
    fn sigaddset(__set: *mut sigset_t, __signo: libc::c_int) -> libc::c_int;
    fn sigprocmask(
        __how: libc::c_int,
        __set: *const sigset_t,
        __oset: *mut sigset_t,
    ) -> libc::c_int;
    fn sigaction(
        __sig: libc::c_int,
        __act: *const sigaction,
        __oact: *mut sigaction,
    ) -> libc::c_int;
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
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __clock_t = libc::c_long;
pub type int64_t = __int64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sigset_t {
    pub __val: [libc::c_ulong; 16],
}
pub type sigset_t = __sigset_t;
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
pub type __sigval_t = sigval;
#[derive(Copy, Clone)]
#[repr(C)]
pub union sigval {
    pub sival_int: libc::c_int,
    pub sival_ptr: *mut libc::c_void,
}
pub type C2RustUnnamed = libc::c_uint;
pub const _SC_SIGSTKSZ: C2RustUnnamed = 250;
pub const _SC_MINSIGSTKSZ: C2RustUnnamed = 249;
pub const _SC_THREAD_ROBUST_PRIO_PROTECT: C2RustUnnamed = 248;
pub const _SC_THREAD_ROBUST_PRIO_INHERIT: C2RustUnnamed = 247;
pub const _SC_XOPEN_STREAMS: C2RustUnnamed = 246;
pub const _SC_TRACE_USER_EVENT_MAX: C2RustUnnamed = 245;
pub const _SC_TRACE_SYS_MAX: C2RustUnnamed = 244;
pub const _SC_TRACE_NAME_MAX: C2RustUnnamed = 243;
pub const _SC_TRACE_EVENT_NAME_MAX: C2RustUnnamed = 242;
pub const _SC_SS_REPL_MAX: C2RustUnnamed = 241;
pub const _SC_V7_LPBIG_OFFBIG: C2RustUnnamed = 240;
pub const _SC_V7_LP64_OFF64: C2RustUnnamed = 239;
pub const _SC_V7_ILP32_OFFBIG: C2RustUnnamed = 238;
pub const _SC_V7_ILP32_OFF32: C2RustUnnamed = 237;
pub const _SC_RAW_SOCKETS: C2RustUnnamed = 236;
pub const _SC_IPV6: C2RustUnnamed = 235;
pub const _SC_LEVEL4_CACHE_LINESIZE: C2RustUnnamed = 199;
pub const _SC_LEVEL4_CACHE_ASSOC: C2RustUnnamed = 198;
pub const _SC_LEVEL4_CACHE_SIZE: C2RustUnnamed = 197;
pub const _SC_LEVEL3_CACHE_LINESIZE: C2RustUnnamed = 196;
pub const _SC_LEVEL3_CACHE_ASSOC: C2RustUnnamed = 195;
pub const _SC_LEVEL3_CACHE_SIZE: C2RustUnnamed = 194;
pub const _SC_LEVEL2_CACHE_LINESIZE: C2RustUnnamed = 193;
pub const _SC_LEVEL2_CACHE_ASSOC: C2RustUnnamed = 192;
pub const _SC_LEVEL2_CACHE_SIZE: C2RustUnnamed = 191;
pub const _SC_LEVEL1_DCACHE_LINESIZE: C2RustUnnamed = 190;
pub const _SC_LEVEL1_DCACHE_ASSOC: C2RustUnnamed = 189;
pub const _SC_LEVEL1_DCACHE_SIZE: C2RustUnnamed = 188;
pub const _SC_LEVEL1_ICACHE_LINESIZE: C2RustUnnamed = 187;
pub const _SC_LEVEL1_ICACHE_ASSOC: C2RustUnnamed = 186;
pub const _SC_LEVEL1_ICACHE_SIZE: C2RustUnnamed = 185;
pub const _SC_TRACE_LOG: C2RustUnnamed = 184;
pub const _SC_TRACE_INHERIT: C2RustUnnamed = 183;
pub const _SC_TRACE_EVENT_FILTER: C2RustUnnamed = 182;
pub const _SC_TRACE: C2RustUnnamed = 181;
pub const _SC_HOST_NAME_MAX: C2RustUnnamed = 180;
pub const _SC_V6_LPBIG_OFFBIG: C2RustUnnamed = 179;
pub const _SC_V6_LP64_OFF64: C2RustUnnamed = 178;
pub const _SC_V6_ILP32_OFFBIG: C2RustUnnamed = 177;
pub const _SC_V6_ILP32_OFF32: C2RustUnnamed = 176;
pub const _SC_2_PBS_CHECKPOINT: C2RustUnnamed = 175;
pub const _SC_STREAMS: C2RustUnnamed = 174;
pub const _SC_SYMLOOP_MAX: C2RustUnnamed = 173;
pub const _SC_2_PBS_TRACK: C2RustUnnamed = 172;
pub const _SC_2_PBS_MESSAGE: C2RustUnnamed = 171;
pub const _SC_2_PBS_LOCATE: C2RustUnnamed = 170;
pub const _SC_2_PBS_ACCOUNTING: C2RustUnnamed = 169;
pub const _SC_2_PBS: C2RustUnnamed = 168;
pub const _SC_USER_GROUPS_R: C2RustUnnamed = 167;
pub const _SC_USER_GROUPS: C2RustUnnamed = 166;
pub const _SC_TYPED_MEMORY_OBJECTS: C2RustUnnamed = 165;
pub const _SC_TIMEOUTS: C2RustUnnamed = 164;
pub const _SC_SYSTEM_DATABASE_R: C2RustUnnamed = 163;
pub const _SC_SYSTEM_DATABASE: C2RustUnnamed = 162;
pub const _SC_THREAD_SPORADIC_SERVER: C2RustUnnamed = 161;
pub const _SC_SPORADIC_SERVER: C2RustUnnamed = 160;
pub const _SC_SPAWN: C2RustUnnamed = 159;
pub const _SC_SIGNALS: C2RustUnnamed = 158;
pub const _SC_SHELL: C2RustUnnamed = 157;
pub const _SC_REGEX_VERSION: C2RustUnnamed = 156;
pub const _SC_REGEXP: C2RustUnnamed = 155;
pub const _SC_SPIN_LOCKS: C2RustUnnamed = 154;
pub const _SC_READER_WRITER_LOCKS: C2RustUnnamed = 153;
pub const _SC_NETWORKING: C2RustUnnamed = 152;
pub const _SC_SINGLE_PROCESS: C2RustUnnamed = 151;
pub const _SC_MULTI_PROCESS: C2RustUnnamed = 150;
pub const _SC_MONOTONIC_CLOCK: C2RustUnnamed = 149;
pub const _SC_FILE_SYSTEM: C2RustUnnamed = 148;
pub const _SC_FILE_LOCKING: C2RustUnnamed = 147;
pub const _SC_FILE_ATTRIBUTES: C2RustUnnamed = 146;
pub const _SC_PIPE: C2RustUnnamed = 145;
pub const _SC_FIFO: C2RustUnnamed = 144;
pub const _SC_FD_MGMT: C2RustUnnamed = 143;
pub const _SC_DEVICE_SPECIFIC_R: C2RustUnnamed = 142;
pub const _SC_DEVICE_SPECIFIC: C2RustUnnamed = 141;
pub const _SC_DEVICE_IO: C2RustUnnamed = 140;
pub const _SC_THREAD_CPUTIME: C2RustUnnamed = 139;
pub const _SC_CPUTIME: C2RustUnnamed = 138;
pub const _SC_CLOCK_SELECTION: C2RustUnnamed = 137;
pub const _SC_C_LANG_SUPPORT_R: C2RustUnnamed = 136;
pub const _SC_C_LANG_SUPPORT: C2RustUnnamed = 135;
pub const _SC_BASE: C2RustUnnamed = 134;
pub const _SC_BARRIERS: C2RustUnnamed = 133;
pub const _SC_ADVISORY_INFO: C2RustUnnamed = 132;
pub const _SC_XOPEN_REALTIME_THREADS: C2RustUnnamed = 131;
pub const _SC_XOPEN_REALTIME: C2RustUnnamed = 130;
pub const _SC_XOPEN_LEGACY: C2RustUnnamed = 129;
pub const _SC_XBS5_LPBIG_OFFBIG: C2RustUnnamed = 128;
pub const _SC_XBS5_LP64_OFF64: C2RustUnnamed = 127;
pub const _SC_XBS5_ILP32_OFFBIG: C2RustUnnamed = 126;
pub const _SC_XBS5_ILP32_OFF32: C2RustUnnamed = 125;
pub const _SC_NL_TEXTMAX: C2RustUnnamed = 124;
pub const _SC_NL_SETMAX: C2RustUnnamed = 123;
pub const _SC_NL_NMAX: C2RustUnnamed = 122;
pub const _SC_NL_MSGMAX: C2RustUnnamed = 121;
pub const _SC_NL_LANGMAX: C2RustUnnamed = 120;
pub const _SC_NL_ARGMAX: C2RustUnnamed = 119;
pub const _SC_USHRT_MAX: C2RustUnnamed = 118;
pub const _SC_ULONG_MAX: C2RustUnnamed = 117;
pub const _SC_UINT_MAX: C2RustUnnamed = 116;
pub const _SC_UCHAR_MAX: C2RustUnnamed = 115;
pub const _SC_SHRT_MIN: C2RustUnnamed = 114;
pub const _SC_SHRT_MAX: C2RustUnnamed = 113;
pub const _SC_SCHAR_MIN: C2RustUnnamed = 112;
pub const _SC_SCHAR_MAX: C2RustUnnamed = 111;
pub const _SC_SSIZE_MAX: C2RustUnnamed = 110;
pub const _SC_NZERO: C2RustUnnamed = 109;
pub const _SC_MB_LEN_MAX: C2RustUnnamed = 108;
pub const _SC_WORD_BIT: C2RustUnnamed = 107;
pub const _SC_LONG_BIT: C2RustUnnamed = 106;
pub const _SC_INT_MIN: C2RustUnnamed = 105;
pub const _SC_INT_MAX: C2RustUnnamed = 104;
pub const _SC_CHAR_MIN: C2RustUnnamed = 103;
pub const _SC_CHAR_MAX: C2RustUnnamed = 102;
pub const _SC_CHAR_BIT: C2RustUnnamed = 101;
pub const _SC_XOPEN_XPG4: C2RustUnnamed = 100;
pub const _SC_XOPEN_XPG3: C2RustUnnamed = 99;
pub const _SC_XOPEN_XPG2: C2RustUnnamed = 98;
pub const _SC_2_UPE: C2RustUnnamed = 97;
pub const _SC_2_C_VERSION: C2RustUnnamed = 96;
pub const _SC_2_CHAR_TERM: C2RustUnnamed = 95;
pub const _SC_XOPEN_SHM: C2RustUnnamed = 94;
pub const _SC_XOPEN_ENH_I18N: C2RustUnnamed = 93;
pub const _SC_XOPEN_CRYPT: C2RustUnnamed = 92;
pub const _SC_XOPEN_UNIX: C2RustUnnamed = 91;
pub const _SC_XOPEN_XCU_VERSION: C2RustUnnamed = 90;
pub const _SC_XOPEN_VERSION: C2RustUnnamed = 89;
pub const _SC_PASS_MAX: C2RustUnnamed = 88;
pub const _SC_ATEXIT_MAX: C2RustUnnamed = 87;
pub const _SC_AVPHYS_PAGES: C2RustUnnamed = 86;
pub const _SC_PHYS_PAGES: C2RustUnnamed = 85;
pub const _SC_NPROCESSORS_ONLN: C2RustUnnamed = 84;
pub const _SC_NPROCESSORS_CONF: C2RustUnnamed = 83;
pub const _SC_THREAD_PROCESS_SHARED: C2RustUnnamed = 82;
pub const _SC_THREAD_PRIO_PROTECT: C2RustUnnamed = 81;
pub const _SC_THREAD_PRIO_INHERIT: C2RustUnnamed = 80;
pub const _SC_THREAD_PRIORITY_SCHEDULING: C2RustUnnamed = 79;
pub const _SC_THREAD_ATTR_STACKSIZE: C2RustUnnamed = 78;
pub const _SC_THREAD_ATTR_STACKADDR: C2RustUnnamed = 77;
pub const _SC_THREAD_THREADS_MAX: C2RustUnnamed = 76;
pub const _SC_THREAD_STACK_MIN: C2RustUnnamed = 75;
pub const _SC_THREAD_KEYS_MAX: C2RustUnnamed = 74;
pub const _SC_THREAD_DESTRUCTOR_ITERATIONS: C2RustUnnamed = 73;
pub const _SC_TTY_NAME_MAX: C2RustUnnamed = 72;
pub const _SC_LOGIN_NAME_MAX: C2RustUnnamed = 71;
pub const _SC_GETPW_R_SIZE_MAX: C2RustUnnamed = 70;
pub const _SC_GETGR_R_SIZE_MAX: C2RustUnnamed = 69;
pub const _SC_THREAD_SAFE_FUNCTIONS: C2RustUnnamed = 68;
pub const _SC_THREADS: C2RustUnnamed = 67;
pub const _SC_T_IOV_MAX: C2RustUnnamed = 66;
pub const _SC_PII_OSI_M: C2RustUnnamed = 65;
pub const _SC_PII_OSI_CLTS: C2RustUnnamed = 64;
pub const _SC_PII_OSI_COTS: C2RustUnnamed = 63;
pub const _SC_PII_INTERNET_DGRAM: C2RustUnnamed = 62;
pub const _SC_PII_INTERNET_STREAM: C2RustUnnamed = 61;
pub const _SC_IOV_MAX: C2RustUnnamed = 60;
pub const _SC_UIO_MAXIOV: C2RustUnnamed = 60;
pub const _SC_SELECT: C2RustUnnamed = 59;
pub const _SC_POLL: C2RustUnnamed = 58;
pub const _SC_PII_OSI: C2RustUnnamed = 57;
pub const _SC_PII_INTERNET: C2RustUnnamed = 56;
pub const _SC_PII_SOCKET: C2RustUnnamed = 55;
pub const _SC_PII_XTI: C2RustUnnamed = 54;
pub const _SC_PII: C2RustUnnamed = 53;
pub const _SC_2_LOCALEDEF: C2RustUnnamed = 52;
pub const _SC_2_SW_DEV: C2RustUnnamed = 51;
pub const _SC_2_FORT_RUN: C2RustUnnamed = 50;
pub const _SC_2_FORT_DEV: C2RustUnnamed = 49;
pub const _SC_2_C_DEV: C2RustUnnamed = 48;
pub const _SC_2_C_BIND: C2RustUnnamed = 47;
pub const _SC_2_VERSION: C2RustUnnamed = 46;
pub const _SC_CHARCLASS_NAME_MAX: C2RustUnnamed = 45;
pub const _SC_RE_DUP_MAX: C2RustUnnamed = 44;
pub const _SC_LINE_MAX: C2RustUnnamed = 43;
pub const _SC_EXPR_NEST_MAX: C2RustUnnamed = 42;
pub const _SC_EQUIV_CLASS_MAX: C2RustUnnamed = 41;
pub const _SC_COLL_WEIGHTS_MAX: C2RustUnnamed = 40;
pub const _SC_BC_STRING_MAX: C2RustUnnamed = 39;
pub const _SC_BC_SCALE_MAX: C2RustUnnamed = 38;
pub const _SC_BC_DIM_MAX: C2RustUnnamed = 37;
pub const _SC_BC_BASE_MAX: C2RustUnnamed = 36;
pub const _SC_TIMER_MAX: C2RustUnnamed = 35;
pub const _SC_SIGQUEUE_MAX: C2RustUnnamed = 34;
pub const _SC_SEM_VALUE_MAX: C2RustUnnamed = 33;
pub const _SC_SEM_NSEMS_MAX: C2RustUnnamed = 32;
pub const _SC_RTSIG_MAX: C2RustUnnamed = 31;
pub const _SC_PAGESIZE: C2RustUnnamed = 30;
pub const _SC_VERSION: C2RustUnnamed = 29;
pub const _SC_MQ_PRIO_MAX: C2RustUnnamed = 28;
pub const _SC_MQ_OPEN_MAX: C2RustUnnamed = 27;
pub const _SC_DELAYTIMER_MAX: C2RustUnnamed = 26;
pub const _SC_AIO_PRIO_DELTA_MAX: C2RustUnnamed = 25;
pub const _SC_AIO_MAX: C2RustUnnamed = 24;
pub const _SC_AIO_LISTIO_MAX: C2RustUnnamed = 23;
pub const _SC_SHARED_MEMORY_OBJECTS: C2RustUnnamed = 22;
pub const _SC_SEMAPHORES: C2RustUnnamed = 21;
pub const _SC_MESSAGE_PASSING: C2RustUnnamed = 20;
pub const _SC_MEMORY_PROTECTION: C2RustUnnamed = 19;
pub const _SC_MEMLOCK_RANGE: C2RustUnnamed = 18;
pub const _SC_MEMLOCK: C2RustUnnamed = 17;
pub const _SC_MAPPED_FILES: C2RustUnnamed = 16;
pub const _SC_FSYNC: C2RustUnnamed = 15;
pub const _SC_SYNCHRONIZED_IO: C2RustUnnamed = 14;
pub const _SC_PRIORITIZED_IO: C2RustUnnamed = 13;
pub const _SC_ASYNCHRONOUS_IO: C2RustUnnamed = 12;
pub const _SC_TIMERS: C2RustUnnamed = 11;
pub const _SC_PRIORITY_SCHEDULING: C2RustUnnamed = 10;
pub const _SC_REALTIME_SIGNALS: C2RustUnnamed = 9;
pub const _SC_SAVED_IDS: C2RustUnnamed = 8;
pub const _SC_JOB_CONTROL: C2RustUnnamed = 7;
pub const _SC_TZNAME_MAX: C2RustUnnamed = 6;
pub const _SC_STREAM_MAX: C2RustUnnamed = 5;
pub const _SC_OPEN_MAX: C2RustUnnamed = 4;
pub const _SC_NGROUPS_MAX: C2RustUnnamed = 3;
pub const _SC_CLK_TCK: C2RustUnnamed = 2;
pub const _SC_CHILD_MAX: C2RustUnnamed = 1;
pub const _SC_ARG_MAX: C2RustUnnamed = 0;
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
    pub c2rust_unnamed: C2RustUnnamed_3,
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
pub struct rt_context {
    pub c2rust_unnamed: C2RustUnnamed_0,
    pub esym_start: *mut Elf64_Sym,
    pub esym_end: *mut Elf64_Sym,
    pub elf_str: *mut libc::c_char,
    pub prog_base: Elf64_Addr,
    pub bounds_start: *mut libc::c_void,
    pub top_func: *mut libc::c_void,
    pub next: *mut rt_context,
    pub num_callers: libc::c_int,
    pub dwarf: libc::c_int,
}
pub type Elf64_Addr = uint64_t;
pub type uint64_t = __uint64_t;
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
pub type Elf64_Xword = uint64_t;
pub type Elf64_Section = uint16_t;
pub type uint16_t = __uint16_t;
pub type Elf64_Word = uint32_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub c2rust_unnamed: C2RustUnnamed_2,
    pub c2rust_unnamed_0: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub dwarf_line: *mut libc::c_uchar,
    pub dwarf_line_end: *mut libc::c_uchar,
    pub dwarf_line_str: *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub stab_sym: *mut Stab_Sym,
    pub stab_sym_end: *mut Stab_Sym,
    pub stab_str: *mut libc::c_char,
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Elf64_Rela {
    pub r_offset: Elf64_Addr,
    pub r_info: Elf64_Xword,
    pub r_addend: Elf64_Sxword,
}
pub type Elf64_Sxword = int64_t;
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
pub union C2RustUnnamed_3 {
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
    pub c2rust_unnamed: C2RustUnnamed_5,
    pub type_0: CType,
    pub c2rust_unnamed_0: C2RustUnnamed_4,
    pub prev: *mut Sym,
    pub prev_tok: *mut Sym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
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
pub union C2RustUnnamed_5 {
    pub c2rust_unnamed: C2RustUnnamed_6,
    pub enum_val: libc::c_longlong,
    pub d: *mut libc::c_int,
    pub cleanup_func: *mut Sym,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_6 {
    pub c: libc::c_int,
    pub c2rust_unnamed: C2RustUnnamed_7,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_7 {
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct TCCSem {
    pub init: libc::c_int,
    pub sem: sem_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union sem_t {
    pub __size: [libc::c_char; 32],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sigaction {
    pub __sigaction_handler: C2RustUnnamed_8,
    pub sa_mask: __sigset_t,
    pub sa_flags: libc::c_int,
    pub sa_restorer: Option::<unsafe extern "C" fn() -> ()>,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_8 {
    pub sa_handler: __sighandler_t,
    pub sa_sigaction: Option::<
        unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    >,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct siginfo_t {
    pub si_signo: libc::c_int,
    pub si_errno: libc::c_int,
    pub si_code: libc::c_int,
    pub __pad0: libc::c_int,
    pub _sifields: C2RustUnnamed_9,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_9 {
    pub _pad: [libc::c_int; 28],
    pub _kill: C2RustUnnamed_18,
    pub _timer: C2RustUnnamed_17,
    pub _rt: C2RustUnnamed_16,
    pub _sigchld: C2RustUnnamed_15,
    pub _sigfault: C2RustUnnamed_12,
    pub _sigpoll: C2RustUnnamed_11,
    pub _sigsys: C2RustUnnamed_10,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_10 {
    pub _call_addr: *mut libc::c_void,
    pub _syscall: libc::c_int,
    pub _arch: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_11 {
    pub si_band: libc::c_long,
    pub si_fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_12 {
    pub si_addr: *mut libc::c_void,
    pub si_addr_lsb: libc::c_short,
    pub _bounds: C2RustUnnamed_13,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_13 {
    pub _addr_bnd: C2RustUnnamed_14,
    pub _pkey: __uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_14 {
    pub _lower: *mut libc::c_void,
    pub _upper: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_15 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_status: libc::c_int,
    pub si_utime: __clock_t,
    pub si_stime: __clock_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_16 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_17 {
    pub si_tid: libc::c_int,
    pub si_overrun: libc::c_int,
    pub si_sigval: __sigval_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_18 {
    pub si_pid: __pid_t,
    pub si_uid: __uid_t,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct rt_frame {
    pub ip: Elf64_Addr,
    pub fp: Elf64_Addr,
    pub sp: Elf64_Addr,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct bt_info {
    pub file: [libc::c_char; 100],
    pub line: libc::c_int,
    pub func: [libc::c_char; 100],
    pub func_pc: Elf64_Addr,
}
pub const DW_LNS_fixed_advance_pc: C2RustUnnamed_24 = 9;
pub const DW_LNS_const_add_pc: C2RustUnnamed_24 = 8;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_19 {
    pub dir_entry: libc::c_uint,
    pub name: *mut libc::c_char,
}
pub const DW_LNS_set_file: C2RustUnnamed_24 = 4;
pub const DW_LNS_advance_line: C2RustUnnamed_24 = 3;
pub const DW_LNS_advance_pc: C2RustUnnamed_24 = 2;
pub const DW_LNE_hi_user: C2RustUnnamed_25 = 255;
pub const DW_LNE_define_file: C2RustUnnamed_25 = 3;
pub const DW_LNE_set_address: C2RustUnnamed_25 = 2;
pub const DW_LNE_end_sequence: C2RustUnnamed_25 = 1;
pub const DW_FORM_udata: C2RustUnnamed_22 = 15;
pub const DW_FORM_data16: C2RustUnnamed_22 = 30;
pub const DW_FORM_data8: C2RustUnnamed_22 = 7;
pub const DW_FORM_data4: C2RustUnnamed_22 = 6;
pub const DW_FORM_data2: C2RustUnnamed_22 = 5;
pub const DW_FORM_data1: C2RustUnnamed_22 = 11;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_20 {
    pub type_0: libc::c_uint,
    pub form: libc::c_uint,
}
pub const DW_LNCT_directory_index: C2RustUnnamed_23 = 2;
pub const DW_FORM_line_strp: C2RustUnnamed_22 = 31;
pub const DW_LNCT_path: C2RustUnnamed_23 = 1;
pub const N_SOL: __stab_debug_code = 132;
pub const N_SO: __stab_debug_code = 100;
pub const N_EINCL: __stab_debug_code = 162;
pub const N_BINCL: __stab_debug_code = 130;
pub const N_SLINE: __stab_debug_code = 68;
pub const N_FUN: __stab_debug_code = 36;
pub const FPE_FLTDIV: C2RustUnnamed_26 = 3;
pub const FPE_INTDIV: C2RustUnnamed_26 = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ucontext_t {
    pub uc_flags: libc::c_ulong,
    pub uc_link: *mut ucontext_t,
    pub uc_stack: stack_t,
    pub uc_mcontext: mcontext_t,
    pub uc_sigmask: sigset_t,
    pub __fpregs_mem: _libc_fpstate,
    pub __ssp: [libc::c_ulonglong; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _libc_fpstate {
    pub cwd: __uint16_t,
    pub swd: __uint16_t,
    pub ftw: __uint16_t,
    pub fop: __uint16_t,
    pub rip: __uint64_t,
    pub rdp: __uint64_t,
    pub mxcsr: __uint32_t,
    pub mxcr_mask: __uint32_t,
    pub _st: [_libc_fpxreg; 8],
    pub _xmm: [_libc_xmmreg; 16],
    pub __glibc_reserved1: [__uint32_t; 24],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _libc_xmmreg {
    pub element: [__uint32_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _libc_fpxreg {
    pub significand: [libc::c_ushort; 4],
    pub exponent: libc::c_ushort,
    pub __glibc_reserved1: [libc::c_ushort; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct mcontext_t {
    pub gregs: gregset_t,
    pub fpregs: fpregset_t,
    pub __reserved1: [libc::c_ulonglong; 8],
}
pub type fpregset_t = *mut _libc_fpstate;
pub type gregset_t = [greg_t; 23];
pub type greg_t = libc::c_longlong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stack_t {
    pub ss_sp: *mut libc::c_void,
    pub ss_flags: libc::c_int,
    pub ss_size: size_t,
}
pub const REG_RBP: C2RustUnnamed_27 = 10;
pub const REG_RIP: C2RustUnnamed_27 = 16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_21 {
    pub secs: *mut *mut Section,
    pub nb_secs: libc::c_int,
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
pub const N_PSYM: __stab_debug_code = 160;
pub const N_LSYM: __stab_debug_code = 128;
pub const N_SSYM: __stab_debug_code = 96;
pub const N_CATCH: __stab_debug_code = 84;
pub const N_MOD2: __stab_debug_code = 80;
pub const N_EHDECL: __stab_debug_code = 80;
pub const N_DEFD: __stab_debug_code = 74;
pub const N_BROWS: __stab_debug_code = 72;
pub const N_BSLINE: __stab_debug_code = 72;
pub const N_DSLINE: __stab_debug_code = 70;
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
pub const N_FNAME: __stab_debug_code = 34;
pub const N_GSYM: __stab_debug_code = 32;
pub type C2RustUnnamed_22 = libc::c_uint;
pub const DW_FORM_GNU_strp_alt: C2RustUnnamed_22 = 7969;
pub const DW_FORM_GNU_ref_alt: C2RustUnnamed_22 = 7968;
pub const DW_FORM_GNU_str_index: C2RustUnnamed_22 = 7938;
pub const DW_FORM_GNU_addr_index: C2RustUnnamed_22 = 7937;
pub const DW_FORM_addrx4: C2RustUnnamed_22 = 44;
pub const DW_FORM_addrx3: C2RustUnnamed_22 = 43;
pub const DW_FORM_addrx2: C2RustUnnamed_22 = 42;
pub const DW_FORM_addrx1: C2RustUnnamed_22 = 41;
pub const DW_FORM_strx4: C2RustUnnamed_22 = 40;
pub const DW_FORM_strx3: C2RustUnnamed_22 = 39;
pub const DW_FORM_strx2: C2RustUnnamed_22 = 38;
pub const DW_FORM_strx1: C2RustUnnamed_22 = 37;
pub const DW_FORM_ref_sup8: C2RustUnnamed_22 = 36;
pub const DW_FORM_rnglistx: C2RustUnnamed_22 = 35;
pub const DW_FORM_loclistx: C2RustUnnamed_22 = 34;
pub const DW_FORM_implicit_const: C2RustUnnamed_22 = 33;
pub const DW_FORM_ref_sig8: C2RustUnnamed_22 = 32;
pub const DW_FORM_strp_sup: C2RustUnnamed_22 = 29;
pub const DW_FORM_ref_sup4: C2RustUnnamed_22 = 28;
pub const DW_FORM_addrx: C2RustUnnamed_22 = 27;
pub const DW_FORM_strx: C2RustUnnamed_22 = 26;
pub const DW_FORM_flag_present: C2RustUnnamed_22 = 25;
pub const DW_FORM_exprloc: C2RustUnnamed_22 = 24;
pub const DW_FORM_sec_offset: C2RustUnnamed_22 = 23;
pub const DW_FORM_indirect: C2RustUnnamed_22 = 22;
pub const DW_FORM_ref_udata: C2RustUnnamed_22 = 21;
pub const DW_FORM_ref8: C2RustUnnamed_22 = 20;
pub const DW_FORM_ref4: C2RustUnnamed_22 = 19;
pub const DW_FORM_ref2: C2RustUnnamed_22 = 18;
pub const DW_FORM_ref1: C2RustUnnamed_22 = 17;
pub const DW_FORM_ref_addr: C2RustUnnamed_22 = 16;
pub const DW_FORM_strp: C2RustUnnamed_22 = 14;
pub const DW_FORM_sdata: C2RustUnnamed_22 = 13;
pub const DW_FORM_flag: C2RustUnnamed_22 = 12;
pub const DW_FORM_block1: C2RustUnnamed_22 = 10;
pub const DW_FORM_block: C2RustUnnamed_22 = 9;
pub const DW_FORM_string: C2RustUnnamed_22 = 8;
pub const DW_FORM_block4: C2RustUnnamed_22 = 4;
pub const DW_FORM_block2: C2RustUnnamed_22 = 3;
pub const DW_FORM_addr: C2RustUnnamed_22 = 1;
pub type C2RustUnnamed_23 = libc::c_uint;
pub const DW_LNCT_hi_user: C2RustUnnamed_23 = 16383;
pub const DW_LNCT_lo_user: C2RustUnnamed_23 = 8192;
pub const DW_LNCT_MD5: C2RustUnnamed_23 = 5;
pub const DW_LNCT_size: C2RustUnnamed_23 = 4;
pub const DW_LNCT_timestamp: C2RustUnnamed_23 = 3;
pub type C2RustUnnamed_24 = libc::c_uint;
pub const DW_LNS_set_isa: C2RustUnnamed_24 = 12;
pub const DW_LNS_set_epilogue_begin: C2RustUnnamed_24 = 11;
pub const DW_LNS_set_prologue_end: C2RustUnnamed_24 = 10;
pub const DW_LNS_set_basic_block: C2RustUnnamed_24 = 7;
pub const DW_LNS_negate_stmt: C2RustUnnamed_24 = 6;
pub const DW_LNS_set_column: C2RustUnnamed_24 = 5;
pub const DW_LNS_copy: C2RustUnnamed_24 = 1;
pub type C2RustUnnamed_25 = libc::c_uint;
pub const DW_LNE_NVIDIA_set_function_name: C2RustUnnamed_25 = 145;
pub const DW_LNE_NVIDIA_inlined_call: C2RustUnnamed_25 = 144;
pub const DW_LNE_lo_user: C2RustUnnamed_25 = 128;
pub const DW_LNE_set_discriminator: C2RustUnnamed_25 = 4;
pub type C2RustUnnamed_26 = libc::c_uint;
pub const FPE_CONDTRAP: C2RustUnnamed_26 = 15;
pub const FPE_FLTUNK: C2RustUnnamed_26 = 14;
pub const FPE_FLTSUB: C2RustUnnamed_26 = 8;
pub const FPE_FLTINV: C2RustUnnamed_26 = 7;
pub const FPE_FLTRES: C2RustUnnamed_26 = 6;
pub const FPE_FLTUND: C2RustUnnamed_26 = 5;
pub const FPE_FLTOVF: C2RustUnnamed_26 = 4;
pub const FPE_INTOVF: C2RustUnnamed_26 = 2;
pub type C2RustUnnamed_27 = libc::c_uint;
pub const REG_CR2: C2RustUnnamed_27 = 22;
pub const REG_OLDMASK: C2RustUnnamed_27 = 21;
pub const REG_TRAPNO: C2RustUnnamed_27 = 20;
pub const REG_ERR: C2RustUnnamed_27 = 19;
pub const REG_CSGSFS: C2RustUnnamed_27 = 18;
pub const REG_EFL: C2RustUnnamed_27 = 17;
pub const REG_RSP: C2RustUnnamed_27 = 15;
pub const REG_RCX: C2RustUnnamed_27 = 14;
pub const REG_RAX: C2RustUnnamed_27 = 13;
pub const REG_RDX: C2RustUnnamed_27 = 12;
pub const REG_RBX: C2RustUnnamed_27 = 11;
pub const REG_RSI: C2RustUnnamed_27 = 9;
pub const REG_RDI: C2RustUnnamed_27 = 8;
pub const REG_R15: C2RustUnnamed_27 = 7;
pub const REG_R14: C2RustUnnamed_27 = 6;
pub const REG_R13: C2RustUnnamed_27 = 5;
pub const REG_R12: C2RustUnnamed_27 = 4;
pub const REG_R11: C2RustUnnamed_27 = 3;
pub const REG_R10: C2RustUnnamed_27 = 2;
pub const REG_R9: C2RustUnnamed_27 = 1;
pub const REG_R8: C2RustUnnamed_27 = 0;
#[inline]
unsafe extern "C" fn read16le(mut p: *mut libc::c_uchar) -> uint16_t {
    return (*p.offset(0 as libc::c_int as isize) as libc::c_int
        | (*p.offset(1 as libc::c_int as isize) as uint16_t as libc::c_int)
            << 8 as libc::c_int) as uint16_t;
}
#[inline]
unsafe extern "C" fn read32le(mut p: *mut libc::c_uchar) -> uint32_t {
    return read16le(p) as uint32_t
        | (read16le(p.offset(2 as libc::c_int as isize)) as uint32_t)
            << 16 as libc::c_int;
}
#[inline]
unsafe extern "C" fn read64le(mut p: *mut libc::c_uchar) -> uint64_t {
    return read32le(p) as uint64_t
        | (read32le(p.offset(4 as libc::c_int as isize)) as uint64_t)
            << 32 as libc::c_int;
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
#[inline]
unsafe extern "C" fn wait_sem(mut p: *mut TCCSem) {
    if (*p).init == 0 {
        sem_init(&mut (*p).sem, 0 as libc::c_int, 1 as libc::c_int as libc::c_uint);
        (*p).init = 1 as libc::c_int;
    }
    while sem_wait(&mut (*p).sem) < 0 as libc::c_int
        && *__errno_location() == 4 as libc::c_int
    {}
}
#[inline]
unsafe extern "C" fn post_sem(mut p: *mut TCCSem) {
    sem_post(&mut (*p).sem);
}
static mut g_rc: *mut rt_context = 0 as *const rt_context as *mut rt_context;
static mut signal_set: libc::c_int = 0;
static mut g_s1: *mut TCCState = 0 as *const TCCState as *mut TCCState;
static mut rt_sem: TCCSem = TCCSem {
    init: 0,
    sem: sem_t { __size: [0; 32] },
};
unsafe extern "C" fn rt_wait_sem() {
    wait_sem(&mut rt_sem);
}
unsafe extern "C" fn rt_post_sem() {
    post_sem(&mut rt_sem);
}
unsafe extern "C" fn rt_mem(
    mut s1: *mut TCCState,
    mut size: libc::c_int,
) -> libc::c_int {
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut ptr_diff: libc::c_int = 0 as libc::c_int;
    size = (size as libc::c_long + sysconf(_SC_PAGESIZE as libc::c_int)) as libc::c_int;
    ptr = tcc_malloc(size as libc::c_ulong);
    (*s1).run_ptr = ptr;
    (*s1).run_size = size as libc::c_uint;
    return ptr_diff;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_relocate(mut s1: *mut TCCState) -> libc::c_int {
    let mut size: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut ptr_diff: libc::c_int = 0;
    if !((*s1).run_ptr).is_null() {
        tcc_enter_state(s1);
        exit(
            (Some(
                _tcc_error_noabort
                    as unsafe extern "C" fn(*const libc::c_char, ...) -> libc::c_int,
            ))
                .expect(
                    "non-null function pointer",
                )(
                b"'tcc_relocate()' twice is no longer supported\0" as *const u8
                    as *const libc::c_char,
            ),
        );
    }
    if (*s1).do_backtrace != 0 {
        tcc_add_symbol(
            s1,
            b"_tcc_backtrace\0" as *const u8 as *const libc::c_char,
            ::core::mem::transmute::<
                Option::<
                    unsafe extern "C" fn(
                        *mut rt_frame,
                        *const libc::c_char,
                        ::core::ffi::VaList,
                    ) -> libc::c_int,
                >,
                *const libc::c_void,
            >(
                Some(
                    _tcc_backtrace
                        as unsafe extern "C" fn(
                            *mut rt_frame,
                            *const libc::c_char,
                            ::core::ffi::VaList,
                        ) -> libc::c_int,
                ),
            ),
        );
    }
    size = tcc_relocate_ex(s1, 0 as *mut libc::c_void, 0 as libc::c_int as libc::c_uint);
    if size < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    ptr_diff = rt_mem(s1, size);
    if ptr_diff < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    ret = tcc_relocate_ex(s1, (*s1).run_ptr, ptr_diff as libc::c_uint);
    if ret == 0 as libc::c_int {
        st_link(s1);
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_run_free(mut s1: *mut TCCState) {
    let mut size: libc::c_uint = 0;
    let mut ptr: *mut libc::c_void = 0 as *mut libc::c_void;
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < (*s1).nb_loaded_dlls {
        let mut ref_0: *mut DLLReference = *((*s1).loaded_dlls).offset(i as isize);
        if !((*ref_0).handle).is_null() {
            dlclose((*ref_0).handle);
        }
        i += 1;
        i;
    }
    ptr = (*s1).run_ptr;
    if ptr.is_null() {
        return;
    }
    st_unlink(s1);
    size = (*s1).run_size;
    protect_pages(
        (ptr as Elf64_Addr)
            .wrapping_add(
                (ptr as Elf64_Addr).wrapping_neg()
                    & (sysconf(_SC_PAGESIZE as libc::c_int)
                        - 1 as libc::c_int as libc::c_long) as Elf64_Addr,
            ) as *mut libc::c_void,
        (size as libc::c_long - sysconf(_SC_PAGESIZE as libc::c_int)) as libc::c_ulong,
        2 as libc::c_int,
    );
    tcc_free(ptr);
}
#[no_mangle]
pub unsafe extern "C" fn tcc_run(
    mut s1: *mut TCCState,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut prog_main: Option::<
        unsafe extern "C" fn(
            libc::c_int,
            *mut *mut libc::c_char,
            *mut *mut libc::c_char,
        ) -> libc::c_int,
    > = None;
    let mut ret: libc::c_int = 0;
    let mut top_sym: *const libc::c_char = 0 as *const libc::c_char;
    let mut main_jb: jmp_buf = [__jmp_buf_tag {
        __jmpbuf: [0; 8],
        __mask_was_saved: 0,
        __saved_mask: __sigset_t { __val: [0; 16] },
    }; 1];
    let mut envp: *mut *mut libc::c_char = environ;
    if (*s1).dflag as libc::c_int & 16 as libc::c_int != 0
        && -(1 as libc::c_int) as Elf64_Addr
            == get_sym_addr(
                s1,
                b"main\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
                1 as libc::c_int,
            )
    {
        return 0 as libc::c_int;
    }
    tcc_add_symbol(
        s1,
        b"__rt_exit\0" as *const u8 as *const libc::c_char,
        ::core::mem::transmute::<
            Option::<unsafe extern "C" fn(*mut rt_frame, libc::c_int) -> ()>,
            *const libc::c_void,
        >(Some(rt_exit as unsafe extern "C" fn(*mut rt_frame, libc::c_int) -> ())),
    );
    if (*s1).nostdlib != 0 {
        top_sym = if !((*s1).elf_entryname).is_null() {
            (*s1).elf_entryname as *const libc::c_char
        } else {
            b"_start\0" as *const u8 as *const libc::c_char
        };
        (*s1).run_main = top_sym;
    } else {
        tcc_add_support(s1, b"runmain.o\0" as *const u8 as *const libc::c_char);
        (*s1).run_main = b"_runmain\0" as *const u8 as *const libc::c_char;
        top_sym = b"main\0" as *const u8 as *const libc::c_char;
    }
    if tcc_relocate(s1) < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    prog_main = ::core::mem::transmute::<
        *mut libc::c_void,
        Option::<
            unsafe extern "C" fn(
                libc::c_int,
                *mut *mut libc::c_char,
                *mut *mut libc::c_char,
            ) -> libc::c_int,
        >,
    >(
        get_sym_addr(s1, (*s1).run_main, 1 as libc::c_int, 1 as libc::c_int)
            as *mut libc::c_void,
    );
    if -(1 as libc::c_int) as Elf64_Addr
        == ::core::mem::transmute::<
            Option::<
                unsafe extern "C" fn(
                    libc::c_int,
                    *mut *mut libc::c_char,
                    *mut *mut libc::c_char,
                ) -> libc::c_int,
            >,
            Elf64_Addr,
        >(prog_main)
    {
        return -(1 as libc::c_int);
    }
    *__errno_location() = 0 as libc::c_int;
    fflush(stdout);
    fflush(stderr);
    ret = _setjmp(
        _tcc_setjmp(
            s1,
            main_jb.as_mut_ptr() as *mut libc::c_void,
            tcc_get_symbol(s1, top_sym),
            ::core::mem::transmute::<
                Option::<unsafe extern "C" fn(*mut __jmp_buf_tag, libc::c_int) -> !>,
                *mut libc::c_void,
            >(
                Some(
                    longjmp as unsafe extern "C" fn(*mut __jmp_buf_tag, libc::c_int) -> !,
                ),
            ),
        ) as *mut __jmp_buf_tag,
    );
    if 0 as libc::c_int == ret {
        ret = prog_main.expect("non-null function pointer")(argc, argv, envp);
    } else if 0xe0e00e0e as libc::c_uint == ret as libc::c_uint {
        ret = 0 as libc::c_int;
    }
    if (*s1).dflag as libc::c_int & 16 as libc::c_int != 0 && ret != 0 {
        fprintf(
            (*s1).ppfp,
            b"[returns %d]\n\0" as *const u8 as *const libc::c_char,
            ret,
        );
        fflush((*s1).ppfp);
    }
    return ret;
}
unsafe extern "C" fn cleanup_symbols(mut s1: *mut TCCState) {
    let mut s: *mut Section = (*s1).c2rust_unnamed.symtab;
    let mut sym_index: libc::c_int = 0;
    let mut end_sym: libc::c_int = ((*s).data_offset)
        .wrapping_div(::core::mem::size_of::<Elf64_Sym>() as libc::c_ulong)
        as libc::c_int;
    (*(*s).hash).data_offset = 0 as libc::c_int as libc::c_ulong;
    (*(*s).link).data_offset = (*(*s).hash).data_offset;
    (*s).data_offset = (*(*s).link).data_offset;
    init_symtab(s);
    sym_index = 1 as libc::c_int;
    while sym_index < end_sym {
        let mut sym: *mut Elf64_Sym = &mut *((*s).data as *mut Elf64_Sym)
            .offset(sym_index as isize) as *mut Elf64_Sym;
        let mut name: *const libc::c_char = ((*(*s).link).data as *mut libc::c_char)
            .offset((*sym).st_name as isize);
        if !((*sym).st_info as libc::c_int >> 4 as libc::c_int == 0 as libc::c_int) {
            put_elf_sym(
                s,
                (*sym).st_value,
                (*sym).st_size,
                (*sym).st_info as libc::c_int,
                (*sym).st_other as libc::c_int,
                (*sym).st_shndx as libc::c_int,
                name,
            );
        }
        sym_index += 1;
        sym_index;
    }
}
unsafe extern "C" fn cleanup_sections(mut s1: *mut TCCState) {
    let mut p: *mut C2RustUnnamed_21 = &mut (*s1).sections as *mut *mut *mut Section
        as *mut libc::c_void as *mut C2RustUnnamed_21;
    let mut i: libc::c_int = 0;
    let mut f: libc::c_int = 2 as libc::c_int;
    loop {
        f -= 1;
        i = f;
        while i < (*p).nb_secs {
            let mut s: *mut Section = *((*p).secs).offset(i as isize);
            if s == (*s1).c2rust_unnamed.symtab
                || s == (*(*s1).c2rust_unnamed.symtab).link
                || s == (*(*s1).c2rust_unnamed.symtab).hash
            {
                (*s).data_allocated = (*s).data_offset;
                (*s)
                    .data = tcc_realloc(
                    (*s).data as *mut libc::c_void,
                    (*s).data_allocated,
                ) as *mut libc::c_uchar;
            } else {
                free_section(s);
                tcc_free(s as *mut libc::c_void);
                let ref mut fresh2 = *((*p).secs).offset(i as isize);
                *fresh2 = 0 as *mut Section;
            }
            i += 1;
            i;
        }
        p = p.offset(1);
        p;
        if !(f != 0) {
            break;
        }
    };
}
unsafe extern "C" fn tcc_relocate_ex(
    mut s1: *mut TCCState,
    mut ptr: *mut libc::c_void,
    mut ptr_diff: libc::c_uint,
) -> libc::c_int {
    let mut s: *mut Section = 0 as *mut Section;
    let mut offset: libc::c_uint = 0;
    let mut length: libc::c_uint = 0;
    let mut align: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut k: libc::c_uint = 0;
    let mut f: libc::c_uint = 0;
    let mut n: libc::c_uint = 0;
    let mut copy: libc::c_uint = 0;
    let mut mem: Elf64_Addr = 0;
    let mut addr: Elf64_Addr = 0;
    if ptr.is_null() {
        tcc_add_runtime(s1);
        resolve_common_syms(s1);
        build_got_entries(s1, 0 as libc::c_int);
    }
    copy = 0 as libc::c_int as libc::c_uint;
    offset = copy;
    mem = ptr as Elf64_Addr;
    loop {
        if (*s1).verbose as libc::c_int == 2 as libc::c_int && copy != 0 {
            printf(
                &*(b"-----------------------------------------------------\n\0"
                    as *const u8 as *const libc::c_char)
                    .offset(
                        (8 as libc::c_int * 2 as libc::c_int - 8 as libc::c_int) as isize,
                    ) as *const libc::c_char,
            );
        }
        if (*s1).nb_errors != 0 {
            return -(1 as libc::c_int);
        }
        if copy == 3 as libc::c_int as libc::c_uint {
            return 0 as libc::c_int;
        }
        let mut current_block_50: u64;
        k = 0 as libc::c_int as libc::c_uint;
        while k < 3 as libc::c_int as libc::c_uint {
            n = 0 as libc::c_int as libc::c_uint;
            addr = 0 as libc::c_int as Elf64_Addr;
            i = 1 as libc::c_int as libc::c_uint;
            while i < (*s1).nb_sections as libc::c_uint {
                static mut shf: [libc::c_char; 3] = [
                    ((1 as libc::c_int) << 1 as libc::c_int
                        | (1 as libc::c_int) << 2 as libc::c_int) as libc::c_char,
                    ((1 as libc::c_int) << 1 as libc::c_int) as libc::c_char,
                    ((1 as libc::c_int) << 1 as libc::c_int
                        | (1 as libc::c_int) << 0 as libc::c_int) as libc::c_char,
                ];
                s = *((*s1).sections).offset(i as isize);
                if !(shf[k as usize] as libc::c_int
                    != (*s).sh_flags
                        & ((1 as libc::c_int) << 1 as libc::c_int
                            | (1 as libc::c_int) << 0 as libc::c_int
                            | (1 as libc::c_int) << 2 as libc::c_int))
                {
                    length = (*s).data_offset as libc::c_uint;
                    if copy == 2 as libc::c_int as libc::c_uint {
                        if addr == 0 as libc::c_int as Elf64_Addr {
                            addr = (*s).sh_addr;
                        }
                        n = ((*s).sh_addr)
                            .wrapping_sub(addr)
                            .wrapping_add(length as Elf64_Addr) as libc::c_uint;
                    } else if copy != 0 {
                        if (*s1).verbose as libc::c_int == 2 as libc::c_int {
                            printf(
                                b"%d: %-16s %p  len %05x  align %04x\n\0" as *const u8
                                    as *const libc::c_char,
                                k,
                                ((*s).name).as_mut_ptr(),
                                (*s).sh_addr as *mut libc::c_void,
                                length,
                                (*s).sh_addralign,
                            );
                        }
                        ptr = (*s).sh_addr as *mut libc::c_void;
                        if k == 0 as libc::c_int as libc::c_uint {
                            ptr = ((*s).sh_addr).wrapping_add(ptr_diff as Elf64_Addr)
                                as *mut libc::c_void;
                        }
                        if ((*s).data).is_null() || (*s).sh_type == 8 as libc::c_int {
                            memset(ptr, 0 as libc::c_int, length as libc::c_ulong);
                        } else {
                            memcpy(
                                ptr,
                                (*s).data as *const libc::c_void,
                                length as libc::c_ulong,
                            );
                        }
                    } else {
                        align = (*s).sh_addralign as libc::c_uint;
                        n = n.wrapping_add(1);
                        if n == 1 as libc::c_int as libc::c_uint {
                            if align < 64 as libc::c_int as libc::c_uint {
                                align = 64 as libc::c_int as libc::c_uint;
                            }
                            if k <= 0 as libc::c_int as libc::c_uint {
                                align = sysconf(_SC_PAGESIZE as libc::c_int)
                                    as libc::c_uint;
                            }
                        }
                        (*s).sh_addralign = align as libc::c_int;
                        addr = if k != 0 {
                            mem.wrapping_add(ptr_diff as Elf64_Addr)
                        } else {
                            mem
                        };
                        offset = (offset as Elf64_Addr)
                            .wrapping_add(
                                addr.wrapping_add(offset as Elf64_Addr).wrapping_neg()
                                    & align.wrapping_sub(1 as libc::c_int as libc::c_uint)
                                        as Elf64_Addr,
                            ) as libc::c_uint as libc::c_uint;
                        (*s)
                            .sh_addr = if mem != 0 {
                            addr.wrapping_add(offset as Elf64_Addr)
                        } else {
                            0 as libc::c_int as Elf64_Addr
                        };
                        offset = offset.wrapping_add(length);
                    }
                }
                i = i.wrapping_add(1);
                i;
            }
            if copy == 2 as libc::c_int as libc::c_uint {
                if !(n == 0 as libc::c_int as libc::c_uint) {
                    f = k;
                    if f >= 0 as libc::c_int as libc::c_uint {
                        if f != 0 as libc::c_int as libc::c_uint {
                            current_block_50 = 12209867499936983673;
                        } else {
                            f = 3 as libc::c_int as libc::c_uint;
                            current_block_50 = 1356832168064818221;
                        }
                    } else {
                        current_block_50 = 1356832168064818221;
                    }
                    match current_block_50 {
                        12209867499936983673 => {}
                        _ => {
                            n = (n as Elf64_Addr)
                                .wrapping_add(
                                    (n as Elf64_Addr).wrapping_neg()
                                        & (sysconf(_SC_PAGESIZE as libc::c_int)
                                            - 1 as libc::c_int as libc::c_long) as Elf64_Addr,
                                ) as libc::c_uint;
                            if (*s1).verbose as libc::c_int == 2 as libc::c_int {
                                printf(
                                    b"protect         %3s %p  len %05x\n\0" as *const u8
                                        as *const libc::c_char,
                                    &*(b"rx\0ro\0rw\0rwx\0" as *const u8 as *const libc::c_char)
                                        .offset(
                                            f.wrapping_mul(3 as libc::c_int as libc::c_uint) as isize,
                                        ) as *const libc::c_char,
                                    addr as *mut libc::c_void,
                                    n,
                                );
                            }
                            if protect_pages(
                                addr as *mut libc::c_void,
                                n as libc::c_ulong,
                                f as libc::c_int,
                            ) < 0 as libc::c_int
                            {
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
                                    b"mprotect failed (did you mean to configure --with-selinux?)\0"
                                        as *const u8 as *const libc::c_char,
                                );
                            }
                        }
                    }
                }
            }
            k = k.wrapping_add(1);
            k;
        }
        if 0 as libc::c_int as Elf64_Addr == mem {
            return (offset as Elf64_Addr)
                .wrapping_add(
                    (offset as Elf64_Addr).wrapping_neg()
                        & (sysconf(_SC_PAGESIZE as libc::c_int)
                            - 1 as libc::c_int as libc::c_long) as Elf64_Addr,
                ) as libc::c_int;
        }
        copy = copy.wrapping_add(1);
        if copy == 2 as libc::c_int as libc::c_uint {
            continue;
        }
        if copy == 3 as libc::c_int as libc::c_uint {
            cleanup_symbols(s1);
            cleanup_sections(s1);
        } else {
            relocate_syms(s1, (*s1).c2rust_unnamed.symtab, 1 as libc::c_int);
            relocate_plt(s1);
            relocate_sections(s1);
        }
    };
}
unsafe extern "C" fn protect_pages(
    mut ptr: *mut libc::c_void,
    mut length: libc::c_ulong,
    mut mode: libc::c_int,
) -> libc::c_int {
    static mut protect: [libc::c_uchar; 4] = [
        (0x1 as libc::c_int | 0x4 as libc::c_int) as libc::c_uchar,
        0x1 as libc::c_int as libc::c_uchar,
        (0x1 as libc::c_int | 0x2 as libc::c_int) as libc::c_uchar,
        (0x1 as libc::c_int | 0x2 as libc::c_int | 0x4 as libc::c_int) as libc::c_uchar,
    ];
    if mprotect(ptr, length, protect[mode as usize] as libc::c_int) != 0 {
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn bt_link(mut s1: *mut TCCState) {
    let mut rc: *mut rt_context = 0 as *mut rt_context;
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    if (*s1).do_backtrace == 0 {
        return;
    }
    rc = tcc_get_symbol(s1, b"__rt_info\0" as *const u8 as *const libc::c_char)
        as *mut rt_context;
    if rc.is_null() {
        return;
    }
    (*rc).esym_start = (*(*s1).c2rust_unnamed.symtab_section).data as *mut Elf64_Sym;
    (*rc)
        .esym_end = ((*(*s1).c2rust_unnamed.symtab_section).data)
        .offset((*(*s1).c2rust_unnamed.symtab_section).data_offset as isize)
        as *mut Elf64_Sym;
    (*rc)
        .elf_str = (*(*(*s1).c2rust_unnamed.symtab_section).link).data
        as *mut libc::c_char;
    if 8 as libc::c_int == 8 as libc::c_int && (*s1).dwarf == 0 {
        (*rc)
            .prog_base = ((*rc).prog_base as libc::c_ulonglong
            & 0xffffffff00000000 as libc::c_ulonglong) as Elf64_Addr;
    }
    if (*s1).do_bounds_check != 0 {
        p = tcc_get_symbol(s1, b"__bound_init\0" as *const u8 as *const libc::c_char);
        if !p.is_null() {
            (::core::mem::transmute::<
                *mut libc::c_void,
                Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>,
            >(p))
                .expect(
                    "non-null function pointer",
                )((*rc).bounds_start, 1 as libc::c_int);
        }
    }
    (*rc).next = g_rc;
    g_rc = rc;
    (*s1).rc = rc;
    if 0 as libc::c_int == signal_set {
        set_exception_handler();
        signal_set = 1 as libc::c_int;
    }
}
unsafe extern "C" fn st_link(mut s1: *mut TCCState) {
    rt_wait_sem();
    (*s1).next = g_s1;
    g_s1 = s1;
    bt_link(s1);
    rt_post_sem();
}
unsafe extern "C" fn ptr_unlink(
    mut list: *mut libc::c_void,
    mut e: *mut libc::c_void,
    mut next: libc::c_uint,
) {
    let mut pp: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut nn: *mut *mut libc::c_void = 0 as *mut *mut libc::c_void;
    let mut p: *mut libc::c_void = 0 as *mut libc::c_void;
    pp = list as *mut *mut libc::c_void;
    loop {
        p = *pp;
        if p.is_null() {
            break;
        }
        nn = (p as *mut libc::c_char).offset(next as isize) as *mut libc::c_void
            as *mut *mut libc::c_void;
        if p == e {
            *pp = *nn;
            break;
        } else {
            pp = nn;
        }
    };
}
unsafe extern "C" fn st_unlink(mut s1: *mut TCCState) {
    rt_wait_sem();
    ptr_unlink(
        &mut g_rc as *mut *mut rt_context as *mut libc::c_void,
        (*s1).rc as *mut libc::c_void,
        &mut (*(0 as *mut rt_context)).next as *mut *mut rt_context as size_t
            as libc::c_uint,
    );
    ptr_unlink(
        &mut g_s1 as *mut *mut TCCState as *mut libc::c_void,
        s1 as *mut libc::c_void,
        &mut (*(0 as *mut TCCState)).next as *mut *mut TCCState as size_t as libc::c_uint,
    );
    rt_post_sem();
}
#[no_mangle]
pub unsafe extern "C" fn _tcc_setjmp(
    mut s1: *mut TCCState,
    mut p_jmp_buf: *mut libc::c_void,
    mut func: *mut libc::c_void,
    mut p_longjmp: *mut libc::c_void,
) -> *mut libc::c_void {
    (*s1).run_lj = p_longjmp;
    (*s1).run_jb = p_jmp_buf;
    if !((*s1).rc).is_null() {
        (*(*s1).rc).top_func = func;
    }
    return p_jmp_buf;
}
#[no_mangle]
pub unsafe extern "C" fn tcc_set_backtrace_func(
    mut s1: *mut TCCState,
    mut data: *mut libc::c_void,
    mut func: Option::<TCCBtFunc>,
) {
    (*s1).bt_func = func;
    (*s1).bt_data = data;
}
unsafe extern "C" fn rt_find_state(mut f: *mut rt_frame) -> *mut TCCState {
    let mut s: *mut TCCState = 0 as *mut TCCState;
    let mut level: libc::c_int = 0;
    let mut pc: Elf64_Addr = 0;
    s = g_s1;
    if s.is_null() || ((*s).next).is_null() {
        return s;
    }
    level = 0 as libc::c_int;
    while level < 8 as libc::c_int {
        if rt_get_caller_pc(&mut pc, f, level) < 0 as libc::c_int {
            break;
        }
        s = g_s1;
        while !s.is_null() {
            if pc >= (*s).run_ptr as Elf64_Addr
                && pc
                    < ((*s).run_ptr as Elf64_Addr)
                        .wrapping_add((*s).run_size as Elf64_Addr)
            {
                return s;
            }
            s = (*s).next;
        }
        level += 1;
        level;
    }
    return 0 as *mut TCCState;
}
unsafe extern "C" fn rt_exit(mut f: *mut rt_frame, mut code: libc::c_int) {
    let mut s: *mut TCCState = 0 as *mut TCCState;
    rt_wait_sem();
    s = rt_find_state(f);
    rt_post_sem();
    if !s.is_null() && !((*s).run_lj).is_null() {
        if code == 0 as libc::c_int {
            code = 0xe0e00e0e as libc::c_uint as libc::c_int;
        }
        (::core::mem::transmute::<
            *mut libc::c_void,
            Option::<unsafe extern "C" fn(*mut libc::c_void, libc::c_int) -> ()>,
        >((*s).run_lj))
            .expect("non-null function pointer")((*s).run_jb, code);
    }
    exit(code);
}
unsafe extern "C" fn rt_vprintf(
    mut fmt: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) -> libc::c_int {
    let mut ret: libc::c_int = vfprintf(stderr, fmt, ap.as_va_list());
    fflush(stderr);
    return ret;
}
unsafe extern "C" fn rt_printf(
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::core::ffi::VaListImpl;
    let mut r: libc::c_int = 0;
    ap = args.clone();
    r = rt_vprintf(fmt, ap.as_va_list());
    return r;
}
unsafe extern "C" fn rt_elfsym(
    mut rc: *mut rt_context,
    mut wanted_pc: Elf64_Addr,
    mut func_addr: *mut Elf64_Addr,
) -> *mut libc::c_char {
    let mut esym: *mut Elf64_Sym = 0 as *mut Elf64_Sym;
    esym = ((*rc).esym_start).offset(1 as libc::c_int as isize);
    while esym < (*rc).esym_end {
        let mut type_0: libc::c_int = (*esym).st_info as libc::c_int
            & 0xf as libc::c_int;
        if (type_0 == 2 as libc::c_int || type_0 == 10 as libc::c_int)
            && wanted_pc >= (*esym).st_value
            && wanted_pc < ((*esym).st_value).wrapping_add((*esym).st_size)
        {
            *func_addr = (*esym).st_value;
            return ((*rc).elf_str).offset((*esym).st_name as isize);
        }
        esym = esym.offset(1);
        esym;
    }
    return 0 as *mut libc::c_char;
}
unsafe extern "C" fn rt_printline(
    mut rc: *mut rt_context,
    mut wanted_pc: Elf64_Addr,
    mut bi: *mut bt_info,
) -> Elf64_Addr {
    let mut current_block: u64;
    let mut func_name: [libc::c_char; 128] = [0; 128];
    let mut func_addr: Elf64_Addr = 0;
    let mut last_pc: Elf64_Addr = 0;
    let mut pc: Elf64_Addr = 0;
    let mut incl_files: [*const libc::c_char; 32] = [0 as *const libc::c_char; 32];
    let mut incl_index: libc::c_int = 0;
    let mut last_incl_index: libc::c_int = 0;
    let mut len: libc::c_int = 0;
    let mut last_line_num: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut str: *const libc::c_char = 0 as *const libc::c_char;
    let mut p: *const libc::c_char = 0 as *const libc::c_char;
    let mut sym: *mut Stab_Sym = 0 as *mut Stab_Sym;
    func_name[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
    func_addr = 0 as libc::c_int as Elf64_Addr;
    incl_index = 0 as libc::c_int;
    last_pc = -(1 as libc::c_int) as Elf64_Addr;
    last_line_num = 1 as libc::c_int;
    last_incl_index = 0 as libc::c_int;
    sym = ((*rc).c2rust_unnamed.c2rust_unnamed.stab_sym)
        .offset(1 as libc::c_int as isize);
    loop {
        if !(sym < (*rc).c2rust_unnamed.c2rust_unnamed.stab_sym_end) {
            current_block = 5330834795799507926;
            break;
        }
        str = ((*rc).c2rust_unnamed.c2rust_unnamed.stab_str)
            .offset((*sym).n_strx as isize);
        pc = (*sym).n_value as Elf64_Addr;
        match (*sym).n_type as libc::c_int {
            68 => {
                if func_addr != 0 {
                    current_block = 11103368345410299747;
                } else {
                    current_block = 2348340725009360863;
                }
            }
            100 | 132 => {
                current_block = 2348340725009360863;
            }
            36 => {
                if (*sym).n_strx == 0 as libc::c_int as libc::c_uint {
                    current_block = 11103368345410299747;
                } else {
                    current_block = 2348340725009360863;
                }
            }
            _ => {
                current_block = 17833034027772472439;
            }
        }
        match current_block {
            2348340725009360863 => {
                pc = pc.wrapping_add((*rc).prog_base);
                current_block = 3898934705613723482;
            }
            11103368345410299747 => {
                pc = pc.wrapping_add(func_addr);
                current_block = 3898934705613723482;
            }
            _ => {}
        }
        match current_block {
            3898934705613723482 => {
                if pc >= wanted_pc && wanted_pc >= last_pc {
                    current_block = 18202189149169841166;
                    break;
                }
            }
            _ => {}
        }
        let mut current_block_33: u64;
        match (*sym).n_type as libc::c_int {
            36 => {
                if (*sym).n_strx == 0 as libc::c_int as libc::c_uint {
                    current_block_33 = 9195004724398151952;
                } else {
                    p = strchr(str, ':' as i32);
                    if p.is_null()
                        || {
                            len = (p.offset_from(str) as libc::c_long
                                + 1 as libc::c_int as libc::c_long) as libc::c_int;
                            len as libc::c_ulong
                                > ::core::mem::size_of::<[libc::c_char; 128]>()
                                    as libc::c_ulong
                        }
                    {
                        len = ::core::mem::size_of::<[libc::c_char; 128]>()
                            as libc::c_ulong as libc::c_int;
                    }
                    pstrcpy(func_name.as_mut_ptr(), len as size_t, str);
                    func_addr = pc;
                    current_block_33 = 10891380440665537214;
                }
            }
            68 => {
                last_pc = pc;
                last_line_num = (*sym).n_desc as libc::c_int;
                last_incl_index = incl_index;
                current_block_33 = 10891380440665537214;
            }
            130 => {
                if incl_index < 32 as libc::c_int {
                    let fresh3 = incl_index;
                    incl_index = incl_index + 1;
                    incl_files[fresh3 as usize] = str;
                }
                current_block_33 = 10891380440665537214;
            }
            162 => {
                if incl_index > 1 as libc::c_int {
                    incl_index -= 1;
                    incl_index;
                }
                current_block_33 = 10891380440665537214;
            }
            100 => {
                incl_index = 0 as libc::c_int;
                if (*sym).n_strx != 0 {
                    len = strlen(str) as libc::c_int;
                    if len > 0 as libc::c_int
                        && *str.offset((len - 1 as libc::c_int) as isize) as libc::c_int
                            != '/' as i32
                    {
                        let fresh4 = incl_index;
                        incl_index = incl_index + 1;
                        incl_files[fresh4 as usize] = str;
                    }
                }
                current_block_33 = 9195004724398151952;
            }
            132 => {
                if incl_index != 0 {
                    incl_files[(incl_index - 1 as libc::c_int) as usize] = str;
                }
                current_block_33 = 10891380440665537214;
            }
            _ => {
                current_block_33 = 10891380440665537214;
            }
        }
        match current_block_33 {
            9195004724398151952 => {
                func_name[0 as libc::c_int as usize] = '\0' as i32 as libc::c_char;
                func_addr = 0 as libc::c_int as Elf64_Addr;
                last_pc = -(1 as libc::c_int) as Elf64_Addr;
            }
            _ => {}
        }
        sym = sym.offset(1);
        sym;
    }
    match current_block {
        5330834795799507926 => {
            last_incl_index = 0 as libc::c_int;
            func_name[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
            func_addr = 0 as libc::c_int as Elf64_Addr;
        }
        _ => {}
    }
    i = last_incl_index;
    if i > 0 as libc::c_int {
        i -= 1;
        pstrcpy(
            ((*bi).file).as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
            incl_files[i as usize],
        );
        (*bi).line = last_line_num;
    }
    pstrcpy(
        ((*bi).func).as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
        func_name.as_mut_ptr(),
    );
    (*bi).func_pc = func_addr;
    return func_addr;
}
unsafe extern "C" fn rt_printline_dwarf(
    mut rc: *mut rt_context,
    mut wanted_pc: Elf64_Addr,
    mut bi: *mut bt_info,
) -> Elf64_Addr {
    let mut current_block: u64;
    let mut ln: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut cp: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut end: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut opcode_length: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut size: libc::c_ulonglong = 0;
    let mut length: libc::c_uint = 0;
    let mut version: libc::c_uchar = 0;
    let mut min_insn_length: libc::c_uint = 0;
    let mut max_ops_per_insn: libc::c_uint = 0;
    let mut line_base: libc::c_int = 0;
    let mut line_range: libc::c_uint = 0;
    let mut opcode_base: libc::c_uint = 0;
    let mut opindex: libc::c_uint = 0;
    let mut col: libc::c_uint = 0;
    let mut i: libc::c_uint = 0;
    let mut j: libc::c_uint = 0;
    let mut len: libc::c_uint = 0;
    let mut value: libc::c_ulonglong = 0;
    let mut entry_format: [C2RustUnnamed_20; 256] = [C2RustUnnamed_20 {
        type_0: 0,
        form: 0,
    }; 256];
    let mut dir_size: libc::c_uint = 0;
    let mut filename_size: libc::c_uint = 0;
    let mut filename_table: [C2RustUnnamed_19; 512] = [C2RustUnnamed_19 {
        dir_entry: 0,
        name: 0 as *mut libc::c_char,
    }; 512];
    let mut last_pc: Elf64_Addr = 0;
    let mut pc: Elf64_Addr = 0;
    let mut func_addr: Elf64_Addr = 0;
    let mut line: libc::c_int = 0;
    let mut filename: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut function: *mut libc::c_char = 0 as *mut libc::c_char;
    filename = 0 as *mut libc::c_char;
    function = 0 as *mut libc::c_char;
    func_addr = 0 as libc::c_int as Elf64_Addr;
    line = 0 as libc::c_int;
    ln = (*rc).c2rust_unnamed.c2rust_unnamed_0.dwarf_line;
    's_72: loop {
        if !(ln < (*rc).c2rust_unnamed.c2rust_unnamed_0.dwarf_line_end) {
            current_block = 8466485602140941539;
            break;
        }
        dir_size = 0 as libc::c_int as libc::c_uint;
        filename_size = 0 as libc::c_int as libc::c_uint;
        last_pc = 0 as libc::c_int as Elf64_Addr;
        pc = 0 as libc::c_int as Elf64_Addr;
        func_addr = 0 as libc::c_int as Elf64_Addr;
        line = 1 as libc::c_int;
        filename = 0 as *mut libc::c_char;
        function = 0 as *mut libc::c_char;
        length = 4 as libc::c_int as libc::c_uint;
        size = (if ln.offset(3 as libc::c_int as isize)
            < (*rc).c2rust_unnamed.c2rust_unnamed_0.dwarf_line_end
        {
            ln = ln.offset(4 as libc::c_int as isize);
            read32le(ln.offset(-(4 as libc::c_int as isize)))
        } else {
            0 as libc::c_int as uint32_t
        }) as libc::c_ulonglong;
        if size == 0xffffffff as libc::c_uint as libc::c_ulonglong {
            length = 8 as libc::c_int as libc::c_uint;
            size = (if ln.offset(7 as libc::c_int as isize)
                < (*rc).c2rust_unnamed.c2rust_unnamed_0.dwarf_line_end
            {
                ln = ln.offset(8 as libc::c_int as isize);
                read64le(ln.offset(-(8 as libc::c_int as isize)))
            } else {
                0 as libc::c_int as uint64_t
            }) as libc::c_ulonglong;
        }
        end = ln.offset(size as isize);
        if end < ln || end > (*rc).c2rust_unnamed.c2rust_unnamed_0.dwarf_line_end {
            current_block = 8466485602140941539;
            break;
        }
        version = (if ln.offset(1 as libc::c_int as isize) < end {
            ln = ln.offset(2 as libc::c_int as isize);
            read16le(ln.offset(-(2 as libc::c_int as isize))) as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uchar;
        if version as libc::c_int >= 5 as libc::c_int {
            ln = ln
                .offset(length.wrapping_add(2 as libc::c_int as libc::c_uint) as isize);
        } else {
            ln = ln.offset(length as isize);
        }
        min_insn_length = (if ln < end {
            let fresh5 = ln;
            ln = ln.offset(1);
            *fresh5 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uint;
        if version as libc::c_int >= 4 as libc::c_int {
            max_ops_per_insn = (if ln < end {
                let fresh6 = ln;
                ln = ln.offset(1);
                *fresh6 as libc::c_int
            } else {
                0 as libc::c_int
            }) as libc::c_uint;
        } else {
            max_ops_per_insn = 1 as libc::c_int as libc::c_uint;
        }
        ln = ln.offset(1);
        ln;
        line_base = if ln < end {
            let fresh7 = ln;
            ln = ln.offset(1);
            *fresh7 as libc::c_int
        } else {
            0 as libc::c_int
        };
        line_base
            |= if line_base >= 0x80 as libc::c_int {
                !(0xff as libc::c_int)
            } else {
                0 as libc::c_int
            };
        line_range = (if ln < end {
            let fresh8 = ln;
            ln = ln.offset(1);
            *fresh8 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uint;
        opcode_base = (if ln < end {
            let fresh9 = ln;
            ln = ln.offset(1);
            *fresh9 as libc::c_int
        } else {
            0 as libc::c_int
        }) as libc::c_uint;
        opcode_length = ln;
        ln = ln
            .offset(opcode_base.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize);
        opindex = 0 as libc::c_int as libc::c_uint;
        if version as libc::c_int >= 5 as libc::c_int {
            col = (if ln < end {
                let fresh10 = ln;
                ln = ln.offset(1);
                *fresh10 as libc::c_int
            } else {
                0 as libc::c_int
            }) as libc::c_uint;
            i = 0 as libc::c_int as libc::c_uint;
            while i < col {
                entry_format[i as usize]
                    .type_0 = dwarf_read_uleb128(&mut ln, end) as libc::c_uint;
                entry_format[i as usize]
                    .form = dwarf_read_uleb128(&mut ln, end) as libc::c_uint;
                i = i.wrapping_add(1);
                i;
            }
            dir_size = dwarf_read_uleb128(&mut ln, end) as libc::c_uint;
            i = 0 as libc::c_int as libc::c_uint;
            's_208: loop {
                if !(i < dir_size) {
                    current_block = 9512719473022792396;
                    break;
                }
                j = 0 as libc::c_int as libc::c_uint;
                while j < col {
                    if entry_format[j as usize].type_0
                        == DW_LNCT_path as libc::c_int as libc::c_uint
                    {
                        if entry_format[j as usize].form
                            != DW_FORM_line_strp as libc::c_int as libc::c_uint
                        {
                            current_block = 14756119570859343421;
                            break 's_208;
                        }
                        if length == 4 as libc::c_int as libc::c_uint {
                            if ln.offset(3 as libc::c_int as isize) < end {
                                ln = ln.offset(4 as libc::c_int as isize);
                                read32le(ln.offset(-(4 as libc::c_int as isize)));
                            } else {};
                        } else {
                            if ln.offset(7 as libc::c_int as isize) < end {
                                ln = ln.offset(8 as libc::c_int as isize);
                                read64le(ln.offset(-(8 as libc::c_int as isize)));
                            } else {};
                        };
                    } else {
                        match entry_format[j as usize].form {
                            11 => {
                                ln = ln.offset(1 as libc::c_int as isize);
                            }
                            5 => {
                                ln = ln.offset(2 as libc::c_int as isize);
                            }
                            6 => {
                                ln = ln.offset(3 as libc::c_int as isize);
                            }
                            7 => {
                                ln = ln.offset(8 as libc::c_int as isize);
                            }
                            30 => {
                                ln = ln.offset(16 as libc::c_int as isize);
                            }
                            15 => {
                                dwarf_read_uleb128(&mut ln, end);
                            }
                            _ => {
                                current_block = 14756119570859343421;
                                break 's_208;
                            }
                        }
                    }
                    j = j.wrapping_add(1);
                    j;
                }
                i = i.wrapping_add(1);
                i;
            }
            match current_block {
                14756119570859343421 => {}
                _ => {
                    col = (if ln < end {
                        let fresh11 = ln;
                        ln = ln.offset(1);
                        *fresh11 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as libc::c_uint;
                    i = 0 as libc::c_int as libc::c_uint;
                    while i < col {
                        entry_format[i as usize]
                            .type_0 = dwarf_read_uleb128(&mut ln, end) as libc::c_uint;
                        entry_format[i as usize]
                            .form = dwarf_read_uleb128(&mut ln, end) as libc::c_uint;
                        i = i.wrapping_add(1);
                        i;
                    }
                    filename_size = dwarf_read_uleb128(&mut ln, end) as libc::c_uint;
                    i = 0 as libc::c_int as libc::c_uint;
                    's_330: loop {
                        if !(i < filename_size) {
                            current_block = 2522825242109451841;
                            break;
                        }
                        j = 0 as libc::c_int as libc::c_uint;
                        while j < col {
                            if entry_format[j as usize].type_0
                                == DW_LNCT_path as libc::c_int as libc::c_uint
                            {
                                if entry_format[j as usize].form
                                    != DW_FORM_line_strp as libc::c_int as libc::c_uint
                                {
                                    current_block = 14756119570859343421;
                                    break 's_330;
                                }
                                value = (if length == 4 as libc::c_int as libc::c_uint {
                                    (if ln.offset(3 as libc::c_int as isize) < end {
                                        ln = ln.offset(4 as libc::c_int as isize);
                                        read32le(ln.offset(-(4 as libc::c_int as isize)))
                                    } else {
                                        0 as libc::c_int as uint32_t
                                    }) as uint64_t
                                } else if ln.offset(7 as libc::c_int as isize) < end {
                                    ln = ln.offset(8 as libc::c_int as isize);
                                    read64le(ln.offset(-(8 as libc::c_int as isize)))
                                } else {
                                    0 as libc::c_int as uint64_t
                                }) as libc::c_ulonglong;
                                if i < 512 as libc::c_int as libc::c_uint {
                                    filename_table[i as usize]
                                        .name = ((*rc)
                                        .c2rust_unnamed
                                        .c2rust_unnamed_0
                                        .dwarf_line_str as *mut libc::c_char)
                                        .offset(value as isize);
                                }
                            } else if entry_format[j as usize].type_0
                                == DW_LNCT_directory_index as libc::c_int as libc::c_uint
                            {
                                match entry_format[j as usize].form {
                                    11 => {
                                        value = (if ln < end {
                                            let fresh12 = ln;
                                            ln = ln.offset(1);
                                            *fresh12 as libc::c_int
                                        } else {
                                            0 as libc::c_int
                                        }) as libc::c_ulonglong;
                                    }
                                    5 => {
                                        value = (if ln.offset(1 as libc::c_int as isize) < end {
                                            ln = ln.offset(2 as libc::c_int as isize);
                                            read16le(ln.offset(-(2 as libc::c_int as isize)))
                                                as libc::c_int
                                        } else {
                                            0 as libc::c_int
                                        }) as libc::c_ulonglong;
                                    }
                                    6 => {
                                        value = (if ln.offset(3 as libc::c_int as isize) < end {
                                            ln = ln.offset(4 as libc::c_int as isize);
                                            read32le(ln.offset(-(4 as libc::c_int as isize)))
                                        } else {
                                            0 as libc::c_int as uint32_t
                                        }) as libc::c_ulonglong;
                                    }
                                    15 => {
                                        value = dwarf_read_uleb128(&mut ln, end)
                                            as libc::c_ulonglong;
                                    }
                                    _ => {
                                        current_block = 14756119570859343421;
                                        break 's_330;
                                    }
                                }
                                if i < 512 as libc::c_int as libc::c_uint {
                                    filename_table[i as usize]
                                        .dir_entry = value as libc::c_uint;
                                }
                            } else {
                                match entry_format[j as usize].form {
                                    11 => {
                                        ln = ln.offset(1 as libc::c_int as isize);
                                    }
                                    5 => {
                                        ln = ln.offset(2 as libc::c_int as isize);
                                    }
                                    6 => {
                                        ln = ln.offset(3 as libc::c_int as isize);
                                    }
                                    7 => {
                                        ln = ln.offset(8 as libc::c_int as isize);
                                    }
                                    30 => {
                                        ln = ln.offset(16 as libc::c_int as isize);
                                    }
                                    15 => {
                                        dwarf_read_uleb128(&mut ln, end);
                                    }
                                    _ => {
                                        current_block = 14756119570859343421;
                                        break 's_330;
                                    }
                                }
                            }
                            j = j.wrapping_add(1);
                            j;
                        }
                        i = i.wrapping_add(1);
                        i;
                    }
                }
            }
        } else {
            while if ln < end {
                let fresh13 = ln;
                ln = ln.offset(1);
                *fresh13 as libc::c_int
            } else {
                0 as libc::c_int
            } != 0
            {
                while if ln < end {
                    let fresh14 = ln;
                    ln = ln.offset(1);
                    *fresh14 as libc::c_int
                } else {
                    0 as libc::c_int
                } != 0
                {}
            }
            while if ln < end {
                let fresh15 = ln;
                ln = ln.offset(1);
                *fresh15 as libc::c_int
            } else {
                0 as libc::c_int
            } != 0
            {
                filename_size = filename_size.wrapping_add(1);
                if filename_size < 512 as libc::c_int as libc::c_uint {
                    filename_table[filename_size
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as usize]
                        .name = (ln as *mut libc::c_char)
                        .offset(-(1 as libc::c_int as isize));
                    while if ln < end {
                        let fresh16 = ln;
                        ln = ln.offset(1);
                        *fresh16 as libc::c_int
                    } else {
                        0 as libc::c_int
                    } != 0
                    {}
                    filename_table[filename_size
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) as usize]
                        .dir_entry = dwarf_read_uleb128(&mut ln, end) as libc::c_uint;
                } else {
                    while if ln < end {
                        let fresh17 = ln;
                        ln = ln.offset(1);
                        *fresh17 as libc::c_int
                    } else {
                        0 as libc::c_int
                    } != 0
                    {}
                    dwarf_read_uleb128(&mut ln, end);
                }
                dwarf_read_uleb128(&mut ln, end);
                dwarf_read_uleb128(&mut ln, end);
            }
            current_block = 2522825242109451841;
        }
        match current_block {
            2522825242109451841 => {
                if filename_size >= 1 as libc::c_int as libc::c_uint {
                    filename = filename_table[0 as libc::c_int as usize].name;
                }
                while ln < end {
                    last_pc = pc;
                    i = (if ln < end {
                        let fresh18 = ln;
                        ln = ln.offset(1);
                        *fresh18 as libc::c_int
                    } else {
                        0 as libc::c_int
                    }) as libc::c_uint;
                    if i >= opcode_base {
                        if max_ops_per_insn == 1 as libc::c_int as libc::c_uint {
                            pc = pc
                                .wrapping_add(
                                    i
                                        .wrapping_sub(opcode_base)
                                        .wrapping_div(line_range)
                                        .wrapping_mul(min_insn_length) as Elf64_Addr,
                                );
                        } else {
                            pc = pc
                                .wrapping_add(
                                    opindex
                                        .wrapping_add(
                                            i.wrapping_sub(opcode_base).wrapping_div(line_range),
                                        )
                                        .wrapping_div(max_ops_per_insn)
                                        .wrapping_mul(min_insn_length) as Elf64_Addr,
                                );
                            opindex = opindex
                                .wrapping_add(
                                    i.wrapping_sub(opcode_base).wrapping_div(line_range),
                                )
                                .wrapping_rem(max_ops_per_insn);
                        }
                        i = (i.wrapping_sub(opcode_base).wrapping_rem(line_range)
                            as libc::c_int + line_base) as libc::c_uint;
                    } else {
                        match i {
                            0 => {
                                len = dwarf_read_uleb128(&mut ln, end) as libc::c_uint;
                                cp = ln;
                                ln = ln.offset(len as isize);
                                if len == 0 as libc::c_int as libc::c_uint {
                                    break;
                                }
                                match if cp < end {
                                    let fresh19 = cp;
                                    cp = cp.offset(1);
                                    *fresh19 as libc::c_int
                                } else {
                                    0 as libc::c_int
                                } {
                                    2 => {
                                        pc = if cp.offset(7 as libc::c_int as isize) < end {
                                            cp = cp.offset(8 as libc::c_int as isize);
                                            read64le(cp.offset(-(8 as libc::c_int as isize)))
                                        } else {
                                            0 as libc::c_int as uint64_t
                                        };
                                        opindex = 0 as libc::c_int as libc::c_uint;
                                    }
                                    3 => {
                                        filename_size = filename_size.wrapping_add(1);
                                        if filename_size < 512 as libc::c_int as libc::c_uint {
                                            filename_table[filename_size
                                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as usize]
                                                .name = (ln as *mut libc::c_char)
                                                .offset(-(1 as libc::c_int as isize));
                                            while if ln < end {
                                                let fresh20 = ln;
                                                ln = ln.offset(1);
                                                *fresh20 as libc::c_int
                                            } else {
                                                0 as libc::c_int
                                            } != 0
                                            {}
                                            filename_table[filename_size
                                                    .wrapping_sub(1 as libc::c_int as libc::c_uint) as usize]
                                                .dir_entry = dwarf_read_uleb128(&mut ln, end)
                                                as libc::c_uint;
                                        } else {
                                            while if ln < end {
                                                let fresh21 = ln;
                                                ln = ln.offset(1);
                                                *fresh21 as libc::c_int
                                            } else {
                                                0 as libc::c_int
                                            } != 0
                                            {}
                                            dwarf_read_uleb128(&mut ln, end);
                                        }
                                        dwarf_read_uleb128(&mut ln, end);
                                        dwarf_read_uleb128(&mut ln, end);
                                    }
                                    254 => {
                                        function = cp as *mut libc::c_char;
                                        func_addr = pc;
                                    }
                                    1 | _ => {}
                                }
                                continue;
                            }
                            2 => {
                                if max_ops_per_insn == 1 as libc::c_int as libc::c_uint {
                                    pc = (pc as uint64_t)
                                        .wrapping_add(
                                            dwarf_read_uleb128(&mut ln, end)
                                                * min_insn_length as uint64_t,
                                        ) as Elf64_Addr as Elf64_Addr;
                                } else {
                                    let mut off: libc::c_ulonglong = dwarf_read_uleb128(
                                        &mut ln,
                                        end,
                                    ) as libc::c_ulonglong;
                                    pc = (pc as libc::c_ulonglong)
                                        .wrapping_add(
                                            (opindex as libc::c_ulonglong)
                                                .wrapping_add(off)
                                                .wrapping_div(max_ops_per_insn as libc::c_ulonglong)
                                                .wrapping_mul(min_insn_length as libc::c_ulonglong),
                                        ) as Elf64_Addr as Elf64_Addr;
                                    opindex = (opindex as libc::c_ulonglong)
                                        .wrapping_add(off)
                                        .wrapping_rem(max_ops_per_insn as libc::c_ulonglong)
                                        as libc::c_uint;
                                }
                                i = 0 as libc::c_int as libc::c_uint;
                            }
                            3 => {
                                line = (line as int64_t + dwarf_read_sleb128(&mut ln, end))
                                    as libc::c_int;
                                continue;
                            }
                            4 => {
                                i = dwarf_read_uleb128(&mut ln, end) as libc::c_uint;
                                i = i
                                    .wrapping_sub(
                                        (i > 0 as libc::c_int as libc::c_uint
                                            && (version as libc::c_int) < 5 as libc::c_int)
                                            as libc::c_int as libc::c_uint,
                                    );
                                if i < 512 as libc::c_int as libc::c_uint
                                    && i < filename_size
                                {
                                    filename = filename_table[i as usize].name;
                                }
                                continue;
                            }
                            8 => {
                                if max_ops_per_insn == 1 as libc::c_int as libc::c_uint {
                                    pc = pc
                                        .wrapping_add(
                                            (255 as libc::c_int as libc::c_uint)
                                                .wrapping_sub(opcode_base)
                                                .wrapping_div(line_range)
                                                .wrapping_mul(min_insn_length) as Elf64_Addr,
                                        );
                                } else {
                                    let mut off_0: libc::c_uint = (255 as libc::c_int
                                        as libc::c_uint)
                                        .wrapping_sub(opcode_base)
                                        .wrapping_div(line_range);
                                    pc = pc
                                        .wrapping_add(
                                            opindex
                                                .wrapping_add(off_0)
                                                .wrapping_div(max_ops_per_insn)
                                                .wrapping_mul(min_insn_length) as Elf64_Addr,
                                        );
                                    opindex = opindex
                                        .wrapping_add(off_0)
                                        .wrapping_rem(max_ops_per_insn);
                                }
                                i = 0 as libc::c_int as libc::c_uint;
                            }
                            9 => {
                                i = (if ln.offset(1 as libc::c_int as isize) < end {
                                    ln = ln.offset(2 as libc::c_int as isize);
                                    read16le(ln.offset(-(2 as libc::c_int as isize)))
                                        as libc::c_int
                                } else {
                                    0 as libc::c_int
                                }) as libc::c_uint;
                                pc = pc.wrapping_add(i as Elf64_Addr);
                                opindex = 0 as libc::c_int as libc::c_uint;
                                i = 0 as libc::c_int as libc::c_uint;
                            }
                            _ => {
                                j = 0 as libc::c_int as libc::c_uint;
                                while j
                                    < *opcode_length
                                        .offset(
                                            i.wrapping_sub(1 as libc::c_int as libc::c_uint) as isize,
                                        ) as libc::c_uint
                                {
                                    dwarf_read_uleb128(&mut ln, end);
                                    j = j.wrapping_add(1);
                                    j;
                                }
                                continue;
                            }
                        }
                    }
                    if pc >= wanted_pc && wanted_pc >= last_pc {
                        current_block = 11626161324566381708;
                        break 's_72;
                    }
                    line = (line as libc::c_uint).wrapping_add(i) as libc::c_int
                        as libc::c_int;
                }
            }
            _ => {}
        }
        ln = end;
    }
    match current_block {
        8466485602140941539 => {
            function = 0 as *mut libc::c_char;
            filename = function;
            func_addr = 0 as libc::c_int as Elf64_Addr;
        }
        _ => {}
    }
    if !filename.is_null() {
        pstrcpy(
            ((*bi).file).as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
            filename,
        );
        (*bi).line = line;
    }
    if !function.is_null() {
        pstrcpy(
            ((*bi).func).as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
            function,
        );
    }
    (*bi).func_pc = func_addr;
    return func_addr;
}
unsafe extern "C" fn _tcc_backtrace(
    mut f: *mut rt_frame,
    mut fmt: *const libc::c_char,
    mut ap: ::core::ffi::VaList,
) -> libc::c_int {
    let mut rc: *mut rt_context = 0 as *mut rt_context;
    let mut rc2: *mut rt_context = 0 as *mut rt_context;
    let mut pc: Elf64_Addr = 0;
    let mut skip: [libc::c_char; 40] = [0; 40];
    let mut msg: [libc::c_char; 200] = [0; 200];
    let mut i: libc::c_int = 0;
    let mut level: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    let mut n: libc::c_int = 0;
    let mut one: libc::c_int = 0;
    let mut a: *const libc::c_char = 0 as *const libc::c_char;
    let mut b: *const libc::c_char = 0 as *const libc::c_char;
    let mut bi: bt_info = bt_info {
        file: [0; 100],
        line: 0,
        func: [0; 100],
        func_pc: 0,
    };
    let mut getinfo: Option::<
        unsafe extern "C" fn(*mut rt_context, Elf64_Addr, *mut bt_info) -> Elf64_Addr,
    > = None;
    skip[0 as libc::c_int as usize] = 0 as libc::c_int as libc::c_char;
    if *fmt.offset(0 as libc::c_int as isize) as libc::c_int == '^' as i32
        && {
            a = fmt.offset(1 as libc::c_int as isize);
            b = strchr(a, *fmt.offset(0 as libc::c_int as isize) as libc::c_int);
            !b.is_null()
        }
    {
        memcpy(
            skip.as_mut_ptr() as *mut libc::c_void,
            a as *const libc::c_void,
            b.offset_from(a) as libc::c_long as libc::c_ulong,
        );
        skip[b.offset_from(a) as libc::c_long
            as usize] = 0 as libc::c_int as libc::c_char;
        fmt = b.offset(1 as libc::c_int as isize);
    }
    one = 0 as libc::c_int;
    if *fmt.offset(0 as libc::c_int as isize) as libc::c_int == '\u{1}' as i32 {
        fmt = fmt.offset(1);
        fmt;
        one = 1 as libc::c_int;
    }
    vsnprintf(
        msg.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 200]>() as libc::c_ulong,
        fmt,
        ap.as_va_list(),
    );
    rt_wait_sem();
    rc = g_rc;
    getinfo = Some(
        rt_printline
            as unsafe extern "C" fn(
                *mut rt_context,
                Elf64_Addr,
                *mut bt_info,
            ) -> Elf64_Addr,
    );
    n = 6 as libc::c_int;
    if !rc.is_null() {
        if (*rc).dwarf != 0 {
            getinfo = Some(
                rt_printline_dwarf
                    as unsafe extern "C" fn(
                        *mut rt_context,
                        Elf64_Addr,
                        *mut bt_info,
                    ) -> Elf64_Addr,
            );
        }
        if (*rc).num_callers != 0 {
            n = (*rc).num_callers;
        }
    }
    level = 0 as libc::c_int;
    i = level;
    while level < n {
        ret = rt_get_caller_pc(&mut pc, f, i);
        if ret == -(1 as libc::c_int) {
            break;
        }
        memset(
            &mut bi as *mut bt_info as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<bt_info>() as libc::c_ulong,
        );
        rc2 = rc;
        while !rc2.is_null() {
            if getinfo.expect("non-null function pointer")(rc2, pc, &mut bi) != 0 {
                break;
            }
            a = rt_elfsym(rc2, pc, &mut bi.func_pc);
            if !a.is_null() {
                pstrcpy(
                    (bi.func).as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 100]>() as libc::c_ulong,
                    a,
                );
                break;
            } else {
                rc2 = (*rc2).next;
            }
        }
        if !(skip[0 as libc::c_int as usize] as libc::c_int != 0
            && !(strstr((bi.file).as_mut_ptr(), skip.as_mut_ptr())).is_null())
        {
            let mut s: *mut TCCState = rt_find_state(f);
            if !s.is_null() && ((*s).bt_func).is_some() {
                ret = ((*s).bt_func)
                    .expect(
                        "non-null function pointer",
                    )(
                    (*s).bt_data,
                    pc as *mut libc::c_void,
                    if bi.file[0 as libc::c_int as usize] as libc::c_int != 0 {
                        (bi.file).as_mut_ptr()
                    } else {
                        0 as *mut libc::c_char
                    },
                    bi.line,
                    if bi.func[0 as libc::c_int as usize] as libc::c_int != 0 {
                        (bi.func).as_mut_ptr()
                    } else {
                        0 as *mut libc::c_char
                    },
                    if level == 0 as libc::c_int {
                        msg.as_mut_ptr()
                    } else {
                        0 as *mut libc::c_char
                    },
                );
                if ret == 0 as libc::c_int {
                    break;
                }
            } else {
                if bi.file[0 as libc::c_int as usize] != 0 {
                    rt_printf(
                        b"%s:%d\0" as *const u8 as *const libc::c_char,
                        (bi.file).as_mut_ptr(),
                        bi.line,
                    );
                } else {
                    rt_printf(
                        b"0x%08llx\0" as *const u8 as *const libc::c_char,
                        pc as libc::c_longlong,
                    );
                }
                rt_printf(
                    b": %s %s\0" as *const u8 as *const libc::c_char,
                    if level != 0 {
                        b"by\0" as *const u8 as *const libc::c_char
                    } else {
                        b"at\0" as *const u8 as *const libc::c_char
                    },
                    if bi.func[0 as libc::c_int as usize] as libc::c_int != 0 {
                        (bi.func).as_mut_ptr() as *const libc::c_char
                    } else {
                        b"???\0" as *const u8 as *const libc::c_char
                    },
                );
                if level == 0 as libc::c_int {
                    rt_printf(
                        b": %s\0" as *const u8 as *const libc::c_char,
                        msg.as_mut_ptr(),
                    );
                    if one != 0 {
                        break;
                    }
                }
                rt_printf(b"\n\0" as *const u8 as *const libc::c_char);
            }
            if !rc2.is_null() && bi.func_pc != 0
                && bi.func_pc == (*rc2).top_func as Elf64_Addr
            {
                break;
            }
            level += 1;
            level;
        }
        i += 1;
        i;
    }
    rt_post_sem();
    return 0 as libc::c_int;
}
unsafe extern "C" fn rt_error(
    mut f: *mut rt_frame,
    mut fmt: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut ap: ::core::ffi::VaListImpl;
    let mut msg: [libc::c_char; 200] = [0; 200];
    let mut ret: libc::c_int = 0;
    ap = args.clone();
    snprintf(
        msg.as_mut_ptr(),
        ::core::mem::size_of::<[libc::c_char; 200]>() as libc::c_ulong,
        b"RUNTIME ERROR: %s\0" as *const u8 as *const libc::c_char,
        fmt,
    );
    ret = _tcc_backtrace(f, msg.as_mut_ptr(), ap.as_va_list());
    return ret;
}
unsafe extern "C" fn rt_getcontext(mut uc: *mut ucontext_t, mut rc: *mut rt_frame) {
    (*rc).ip = (*uc).uc_mcontext.gregs[REG_RIP as libc::c_int as usize] as Elf64_Addr;
    (*rc).fp = (*uc).uc_mcontext.gregs[REG_RBP as libc::c_int as usize] as Elf64_Addr;
}
unsafe extern "C" fn sig_error(
    mut signum: libc::c_int,
    mut siginf: *mut siginfo_t,
    mut puc: *mut libc::c_void,
) {
    let mut f: rt_frame = rt_frame { ip: 0, fp: 0, sp: 0 };
    rt_getcontext(puc as *mut ucontext_t, &mut f);
    match signum {
        8 => {
            match (*siginf).si_code {
                1 | 3 => {
                    rt_error(
                        &mut f as *mut rt_frame,
                        b"division by zero\0" as *const u8 as *const libc::c_char,
                    );
                }
                _ => {
                    rt_error(
                        &mut f as *mut rt_frame,
                        b"floating point exception\0" as *const u8 as *const libc::c_char,
                    );
                }
            }
        }
        7 | 11 => {
            rt_error(
                &mut f as *mut rt_frame,
                b"invalid memory access\0" as *const u8 as *const libc::c_char,
            );
        }
        4 => {
            rt_error(
                &mut f as *mut rt_frame,
                b"illegal instruction\0" as *const u8 as *const libc::c_char,
            );
        }
        6 => {
            rt_error(
                &mut f as *mut rt_frame,
                b"abort() called\0" as *const u8 as *const libc::c_char,
            );
        }
        _ => {
            rt_error(
                &mut f as *mut rt_frame,
                b"caught signal %d\0" as *const u8 as *const libc::c_char,
                signum,
            );
        }
    }
    let mut s: sigset_t = __sigset_t { __val: [0; 16] };
    sigemptyset(&mut s);
    sigaddset(&mut s, signum);
    sigprocmask(1 as libc::c_int, &mut s, 0 as *mut sigset_t);
    rt_exit(&mut f, 255 as libc::c_int);
}
unsafe extern "C" fn set_exception_handler() {
    let mut sigact: sigaction = sigaction {
        __sigaction_handler: C2RustUnnamed_8 {
            sa_handler: None,
        },
        sa_mask: __sigset_t { __val: [0; 16] },
        sa_flags: 0,
        sa_restorer: None,
    };
    sigemptyset(&mut sigact.sa_mask);
    sigact.sa_flags = 4 as libc::c_int;
    sigact
        .__sigaction_handler
        .sa_sigaction = Some(
        sig_error
            as unsafe extern "C" fn(libc::c_int, *mut siginfo_t, *mut libc::c_void) -> (),
    );
    sigaction(8 as libc::c_int, &mut sigact, 0 as *mut sigaction);
    sigaction(4 as libc::c_int, &mut sigact, 0 as *mut sigaction);
    sigaction(11 as libc::c_int, &mut sigact, 0 as *mut sigaction);
    sigaction(7 as libc::c_int, &mut sigact, 0 as *mut sigaction);
    sigaction(6 as libc::c_int, &mut sigact, 0 as *mut sigaction);
}
unsafe extern "C" fn rt_get_caller_pc(
    mut paddr: *mut Elf64_Addr,
    mut rc: *mut rt_frame,
    mut level: libc::c_int,
) -> libc::c_int {
    if level == 0 as libc::c_int {
        *paddr = (*rc).ip;
    } else {
        let mut fp: Elf64_Addr = (*rc).fp;
        loop {
            if fp < 0x1000 as libc::c_int as Elf64_Addr {
                return -(1 as libc::c_int);
            }
            level -= 1;
            if 0 as libc::c_int == level {
                break;
            }
            fp = *(fp as *mut Elf64_Addr).offset(0 as libc::c_int as isize);
        }
        *paddr = *(fp as *mut Elf64_Addr).offset(1 as libc::c_int as isize);
    }
    return 0 as libc::c_int;
}
