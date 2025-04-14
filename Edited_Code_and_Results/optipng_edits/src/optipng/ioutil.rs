use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn rename(__old: *const libc::c_char, __new: *const libc::c_char) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
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
    fn fgetpos(__stream: *mut FILE, __pos: *mut fpos_t) -> libc::c_int;
    fn fsetpos(__stream: *mut FILE, __pos: *const fpos_t) -> libc::c_int;
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strpbrk(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn chown(
        __file: *const libc::c_char,
        __owner: __uid_t,
        __group: __gid_t,
    ) -> libc::c_int;
    fn unlink(__name: *const libc::c_char) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn fstat(__fd: libc::c_int, __buf: *mut stat) -> libc::c_int;
    fn chmod(__file: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn mkdir(__path: *const libc::c_char, __mode: __mode_t) -> libc::c_int;
    fn utimensat(
        __fd: libc::c_int,
        __path: *const libc::c_char,
        __times: *const timespec,
        __flags: libc::c_int,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __mbstate_t {
    pub __count: libc::c_int,
    pub __value: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __wch: libc::c_uint,
    pub __wchb: [libc::c_char; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _G_fpos_t {
    pub __pos: __off_t,
    pub __state: __mbstate_t,
}
pub type __fpos_t = _G_fpos_t;
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
pub type fpos_t = __fpos_t;
pub type opng_foffset_t = libc::c_long;
pub type opng_fsize_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[no_mangle]
pub unsafe extern "C" fn opng_ftello(mut stream: *mut FILE) -> opng_foffset_t {
    return ftell(stream);
}
#[no_mangle]
pub unsafe extern "C" fn opng_fseeko(
    mut stream: *mut FILE,
    mut offset: opng_foffset_t,
    mut whence: libc::c_int,
) -> libc::c_int {
    return fseek(stream, offset, whence);
}
#[no_mangle]
pub unsafe extern "C" fn opng_freado(
    mut stream: *mut FILE,
    mut offset: opng_foffset_t,
    mut whence: libc::c_int,
    mut block: *mut libc::c_void,
    mut blocksize: size_t,
) -> size_t {
    let mut pos: fpos_t = _G_fpos_t {
        __pos: 0,
        __state: __mbstate_t {
            __count: 0,
            __value: C2RustUnnamed { __wch: 0 },
        },
    };
    let mut result: size_t = 0;
    if fgetpos(stream, &mut pos) != 0 as libc::c_int {
        return 0 as libc::c_int as size_t;
    }
    if opng_fseeko(stream, offset, whence) == 0 as libc::c_int {
        result = fread(block, 1 as libc::c_int as libc::c_ulong, blocksize, stream);
    } else {
        result = 0 as libc::c_int as size_t;
    }
    if fsetpos(stream, &mut pos) != 0 as libc::c_int {
        result = 0 as libc::c_int as size_t;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn opng_fwriteo(
    mut stream: *mut FILE,
    mut offset: opng_foffset_t,
    mut whence: libc::c_int,
    mut block: *const libc::c_void,
    mut blocksize: size_t,
) -> size_t {
    let mut pos: fpos_t = _G_fpos_t {
        __pos: 0,
        __state: __mbstate_t {
            __count: 0,
            __value: C2RustUnnamed { __wch: 0 },
        },
    };
    let mut result: size_t = 0;
    if fgetpos(stream, &mut pos) != 0 as libc::c_int
        || fflush(stream) != 0 as libc::c_int
    {
        return 0 as libc::c_int as size_t;
    }
    if opng_fseeko(stream, offset, whence) == 0 as libc::c_int {
        result = fwrite(block, 1 as libc::c_int as libc::c_ulong, blocksize, stream);
    } else {
        result = 0 as libc::c_int as size_t;
    }
    if fflush(stream) != 0 as libc::c_int {
        result = 0 as libc::c_int as size_t;
    }
    if fsetpos(stream, &mut pos) != 0 as libc::c_int {
        result = 0 as libc::c_int as size_t;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn opng_fgetsize(
    mut stream: *mut FILE,
    mut size: *mut opng_fsize_t,
) -> libc::c_int {
    let mut sbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if fstat(fileno(stream), &mut sbuf) != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if sbuf.st_size < 0 as libc::c_int as __off_t {
        return -(1 as libc::c_int);
    }
    *size = sbuf.st_size as opng_fsize_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn opng_path_replace_dir(
    mut buffer: *mut libc::c_char,
    mut bufsize: size_t,
    mut old_path: *const libc::c_char,
    mut new_dirname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut path: *const libc::c_char = 0 as *const libc::c_char;
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut dirlen: size_t = 0;
    path = old_path;
    loop {
        ptr = strpbrk(path, b"/\0" as *const u8 as *const libc::c_char);
        if ptr.is_null() {
            break;
        }
        path = ptr.offset(1 as libc::c_int as isize);
    }
    dirlen = strlen(new_dirname);
    if dirlen.wrapping_add(strlen(path)).wrapping_add(2 as libc::c_int as libc::c_ulong)
        >= bufsize
    {
        return 0 as *mut libc::c_char;
    }
    if dirlen > 0 as libc::c_int as size_t {
        strcpy(buffer, new_dirname);
        if (strchr(
            b"/\0" as *const u8 as *const libc::c_char,
            *buffer.offset(dirlen.wrapping_sub(1 as libc::c_int as size_t) as isize)
                as libc::c_int,
        ))
            .is_null()
        {
            let fresh0 = dirlen;
            dirlen = dirlen.wrapping_add(1);
            *buffer.offset(fresh0 as isize) = '/' as i32 as libc::c_char;
        }
    }
    strcpy(buffer.offset(dirlen as isize), path);
    return buffer;
}
#[no_mangle]
pub unsafe extern "C" fn opng_path_replace_ext(
    mut buffer: *mut libc::c_char,
    mut bufsize: size_t,
    mut old_path: *const libc::c_char,
    mut new_extname: *const libc::c_char,
) -> *mut libc::c_char {
    let mut i: size_t = 0;
    let mut pos: size_t = 0;
    if *new_extname.offset(0 as libc::c_int as isize) as libc::c_int != '.' as i32 {
        return 0 as *mut libc::c_char;
    }
    i = 0 as libc::c_int as size_t;
    pos = -(1 as libc::c_int) as size_t;
    while *old_path.offset(i as isize) as libc::c_int != '\0' as i32 {
        if i >= bufsize {
            return 0 as *mut libc::c_char;
        }
        let ref mut fresh1 = *buffer.offset(i as isize);
        *fresh1 = *old_path.offset(i as isize);
        if *fresh1 as libc::c_int == '.' as i32 {
            pos = i;
        }
        i = i.wrapping_add(1);
        i;
    }
    if i > pos {
        i = pos;
    }
    loop {
        if i >= bufsize {
            return 0 as *mut libc::c_char;
        }
        let ref mut fresh2 = *buffer.offset(i as isize);
        *fresh2 = *new_extname;
        if *fresh2 as libc::c_int == '\0' as i32 {
            return buffer;
        }
        i = i.wrapping_add(1);
        i;
        new_extname = new_extname.offset(1);
        new_extname;
    };
}
#[no_mangle]
pub unsafe extern "C" fn opng_path_make_backup(
    mut buffer: *mut libc::c_char,
    mut bufsize: size_t,
    mut path: *const libc::c_char,
) -> *mut libc::c_char {
    static mut bak_extname: [libc::c_char; 5] = unsafe {
        *::core::mem::transmute::<&[u8; 5], &[libc::c_char; 5]>(b".bak\0")
    };
    if (strlen(path))
        .wrapping_add(::core::mem::size_of::<[libc::c_char; 5]>() as libc::c_ulong)
        > bufsize
    {
        return 0 as *mut libc::c_char;
    }
    strcpy(buffer, path);
    strcat(buffer, bak_extname.as_ptr());
    return buffer;
}
#[no_mangle]
pub unsafe extern "C" fn opng_os_rename(
    mut src_path: *const libc::c_char,
    mut dest_path: *const libc::c_char,
    mut clobber: libc::c_int,
) -> libc::c_int {
    if clobber == 0 {
        if access(dest_path, 0 as libc::c_int) >= 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    return rename(src_path, dest_path);
}
#[no_mangle]
pub unsafe extern "C" fn opng_os_create_dir(
    mut dirname: *const libc::c_char,
) -> libc::c_int {
    if *dirname.offset(0 as libc::c_int as isize) as libc::c_int == '\0' as i32 {
        return 0 as libc::c_int;
    }
    let mut sbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if stat(dirname, &mut sbuf) == 0 as libc::c_int {
        return if sbuf.st_mode & 0o40000 as libc::c_int as __mode_t != 0 {
            0 as libc::c_int
        } else {
            -(1 as libc::c_int)
        };
    }
    return mkdir(dirname, 0o777 as libc::c_int as __mode_t);
}
#[no_mangle]
pub unsafe extern "C" fn opng_os_copy_file_attr(
    mut src_path: *const libc::c_char,
    mut dest_path: *const libc::c_char,
) -> libc::c_int {
    let mut sbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut result: libc::c_int = 0;
    if stat(src_path, &mut sbuf) != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    result = 0 as libc::c_int;
    chown(dest_path, sbuf.st_uid, sbuf.st_gid) != 0 as libc::c_int;
    if chmod(dest_path, sbuf.st_mode) != 0 as libc::c_int {
        result = -(1 as libc::c_int);
    }
    let mut times: [timespec; 2] = [timespec { tv_sec: 0, tv_nsec: 0 }; 2];
    times[0 as libc::c_int as usize] = sbuf.st_atim;
    times[1 as libc::c_int as usize] = sbuf.st_mtim;
    if utimensat(
        -(100 as libc::c_int),
        dest_path,
        times.as_mut_ptr() as *const timespec,
        0 as libc::c_int,
    ) != 0 as libc::c_int
    {
        result = -(1 as libc::c_int);
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn opng_os_test_file_access(
    mut path: *const libc::c_char,
    mut mode: *const libc::c_char,
) -> libc::c_int {
    let mut faccess: libc::c_int = 0;
    let mut freg: libc::c_int = 0;
    freg = 0 as libc::c_int;
    faccess = freg;
    if !(strchr(mode, 'f' as i32)).is_null() {
        freg = 1 as libc::c_int;
    }
    if !(strchr(mode, 'r' as i32)).is_null() {
        faccess |= 4 as libc::c_int;
    }
    if !(strchr(mode, 'w' as i32)).is_null() {
        faccess |= 2 as libc::c_int;
    }
    if !(strchr(mode, 'x' as i32)).is_null() {
        faccess |= 1 as libc::c_int;
    }
    if faccess == 0 as libc::c_int && freg == 0 {
        if (strchr(mode, 'e' as i32)).is_null() {
            return 0 as libc::c_int;
        }
    }
    let mut sbuf: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if stat(path, &mut sbuf) != 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if freg != 0
        && sbuf.st_mode & 0o100000 as libc::c_int as __mode_t
            != 0o100000 as libc::c_int as __mode_t
    {
        return -(1 as libc::c_int);
    }
    if faccess == 0 as libc::c_int {
        return 0 as libc::c_int;
    }
    return access(path, faccess);
}
#[no_mangle]
pub unsafe extern "C" fn opng_os_test_file_equiv(
    mut path1: *const libc::c_char,
    mut path2: *const libc::c_char,
) -> libc::c_int {
    let mut sbuf1: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut sbuf2: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    if stat(path1, &mut sbuf1) != 0 as libc::c_int
        || stat(path2, &mut sbuf2) != 0 as libc::c_int
    {
        return -(1 as libc::c_int);
    }
    if sbuf1.st_dev == sbuf2.st_dev && sbuf1.st_ino == sbuf2.st_ino {
        return if sbuf1.st_ino != 0 as libc::c_int as __ino_t {
            1 as libc::c_int
        } else {
            -(1 as libc::c_int)
        }
    } else {
        return 0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn opng_os_unlink(mut path: *const libc::c_char) -> libc::c_int {
    return unlink(path);
}
