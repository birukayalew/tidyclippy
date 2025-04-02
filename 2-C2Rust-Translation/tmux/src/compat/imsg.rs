use ::libc;
extern "C" {
    fn __cmsg_nxthdr(__mhdr: *mut msghdr, __cmsg: *mut cmsghdr) -> *mut cmsghdr;
    fn recvmsg(
        __fd: libc::c_int,
        __message: *mut msghdr,
        __flags: libc::c_int,
    ) -> ssize_t;
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
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
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn getpid() -> __pid_t;
    fn getdtablesize() -> libc::c_int;
    fn getdtablecount() -> libc::c_int;
    fn freezero(_: *mut libc::c_void, _: size_t);
    fn ibuf_dynamic(_: size_t, _: size_t) -> *mut ibuf;
    fn ibuf_add(_: *mut ibuf, _: *const libc::c_void, _: size_t) -> libc::c_int;
    fn ibuf_close(_: *mut msgbuf, _: *mut ibuf);
    fn ibuf_free(_: *mut ibuf);
    fn msgbuf_init(_: *mut msgbuf);
    fn msgbuf_clear(_: *mut msgbuf);
    fn msgbuf_write(_: *mut msgbuf) -> libc::c_int;
}
pub type __u_char = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __pid_t = libc::c_int;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type u_char = __u_char;
pub type pid_t = __pid_t;
pub type ssize_t = __ssize_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct iovec {
    pub iov_base: *mut libc::c_void,
    pub iov_len: size_t,
}
pub type socklen_t = __socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msghdr {
    pub msg_name: *mut libc::c_void,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut iovec,
    pub msg_iovlen: size_t,
    pub msg_control: *mut libc::c_void,
    pub msg_controllen: size_t,
    pub msg_flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cmsghdr {
    pub cmsg_len: size_t,
    pub cmsg_level: libc::c_int,
    pub cmsg_type: libc::c_int,
    pub __cmsg_data: [libc::c_uchar; 0],
}
pub type C2RustUnnamed = libc::c_uint;
pub const SCM_PIDFD: C2RustUnnamed = 4;
pub const SCM_SECURITY: C2RustUnnamed = 3;
pub const SCM_CREDENTIALS: C2RustUnnamed = 2;
pub const SCM_RIGHTS: C2RustUnnamed = 1;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ibuf {
    pub entry: C2RustUnnamed_0,
    pub buf: *mut u_char,
    pub size: size_t,
    pub max: size_t,
    pub wpos: size_t,
    pub rpos: size_t,
    pub fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub tqe_next: *mut ibuf,
    pub tqe_prev: *mut *mut ibuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct msgbuf {
    pub bufs: C2RustUnnamed_1,
    pub queued: uint32_t,
    pub fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub tqh_first: *mut ibuf,
    pub tqh_last: *mut *mut ibuf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ibuf_read {
    pub buf: [u_char; 65535],
    pub rptr: *mut u_char,
    pub wpos: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct imsg_fd {
    pub entry: C2RustUnnamed_2,
    pub fd: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub tqe_next: *mut imsg_fd,
    pub tqe_prev: *mut *mut imsg_fd,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct imsgbuf {
    pub fds: C2RustUnnamed_3,
    pub r: ibuf_read,
    pub w: msgbuf,
    pub fd: libc::c_int,
    pub pid: pid_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_3 {
    pub tqh_first: *mut imsg_fd,
    pub tqh_last: *mut *mut imsg_fd,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct imsg_hdr {
    pub type_0: uint32_t,
    pub len: uint16_t,
    pub flags: uint16_t,
    pub peerid: uint32_t,
    pub pid: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct imsg {
    pub hdr: imsg_hdr,
    pub fd: libc::c_int,
    pub data: *mut libc::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_4 {
    pub hdr: cmsghdr,
    pub buf: [libc::c_char; 24],
}
#[no_mangle]
pub static mut imsg_fd_overhead: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn imsg_init(mut ibuf: *mut imsgbuf, mut fd: libc::c_int) {
    msgbuf_init(&mut (*ibuf).w);
    memset(
        &mut (*ibuf).r as *mut ibuf_read as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ibuf_read>() as libc::c_ulong,
    );
    (*ibuf).fd = fd;
    (*ibuf).w.fd = fd;
    (*ibuf).pid = getpid();
    (*ibuf).fds.tqh_first = 0 as *mut imsg_fd;
    (*ibuf).fds.tqh_last = &mut (*ibuf).fds.tqh_first;
}
#[no_mangle]
pub unsafe extern "C" fn imsg_read(mut ibuf: *mut imsgbuf) -> ssize_t {
    let mut msg: msghdr = msghdr {
        msg_name: 0 as *mut libc::c_void,
        msg_namelen: 0,
        msg_iov: 0 as *mut iovec,
        msg_iovlen: 0,
        msg_control: 0 as *mut libc::c_void,
        msg_controllen: 0,
        msg_flags: 0,
    };
    let mut cmsg: *mut cmsghdr = 0 as *mut cmsghdr;
    let mut cmsgbuf: C2RustUnnamed_4 = C2RustUnnamed_4 {
        hdr: cmsghdr {
            cmsg_len: 0,
            cmsg_level: 0,
            cmsg_type: 0,
            __cmsg_data: [],
        },
    };
    let mut iov: iovec = iovec {
        iov_base: 0 as *mut libc::c_void,
        iov_len: 0,
    };
    let mut n: ssize_t = -(1 as libc::c_int) as ssize_t;
    let mut fd: libc::c_int = 0;
    let mut ifd: *mut imsg_fd = 0 as *mut imsg_fd;
    memset(
        &mut msg as *mut msghdr as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<msghdr>() as libc::c_ulong,
    );
    memset(
        &mut cmsgbuf as *mut C2RustUnnamed_4 as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<C2RustUnnamed_4>() as libc::c_ulong,
    );
    iov
        .iov_base = ((*ibuf).r.buf).as_mut_ptr().offset((*ibuf).r.wpos as isize)
        as *mut libc::c_void;
    iov
        .iov_len = (::core::mem::size_of::<[u_char; 65535]>() as libc::c_ulong)
        .wrapping_sub((*ibuf).r.wpos);
    msg.msg_iov = &mut iov;
    msg.msg_iovlen = 1 as libc::c_int as size_t;
    msg.msg_control = &mut cmsgbuf.buf as *mut [libc::c_char; 24] as *mut libc::c_void;
    msg.msg_controllen = ::core::mem::size_of::<[libc::c_char; 24]>() as libc::c_ulong;
    ifd = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<imsg_fd>() as libc::c_ulong,
    ) as *mut imsg_fd;
    if ifd.is_null() {
        return -(1 as libc::c_int) as ssize_t;
    }
    loop {
        if getdtablecount() + imsg_fd_overhead
            + ((::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                .wrapping_add(::core::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                & !(::core::mem::size_of::<size_t>() as libc::c_ulong)
                    .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                .wrapping_add(
                    (::core::mem::size_of::<cmsghdr>() as libc::c_ulong)
                        .wrapping_add(::core::mem::size_of::<size_t>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        & !(::core::mem::size_of::<size_t>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                )
                .wrapping_sub(
                    ((0 as libc::c_int as libc::c_ulong)
                        .wrapping_add(::core::mem::size_of::<size_t>() as libc::c_ulong)
                        .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                        & !(::core::mem::size_of::<size_t>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong))
                        .wrapping_add(
                            (::core::mem::size_of::<cmsghdr>() as libc::c_ulong)
                                .wrapping_add(
                                    ::core::mem::size_of::<size_t>() as libc::c_ulong,
                                )
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong)
                                & !(::core::mem::size_of::<size_t>() as libc::c_ulong)
                                    .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        ),
                )
                .wrapping_div(::core::mem::size_of::<libc::c_int>() as libc::c_ulong)
                as libc::c_int >= getdtablesize()
        {
            *__errno_location() = 11 as libc::c_int;
            free(ifd as *mut libc::c_void);
            return -(1 as libc::c_int) as ssize_t;
        }
        n = recvmsg((*ibuf).fd, &mut msg, 0 as libc::c_int);
        if n == -(1 as libc::c_int) as ssize_t {
            if !(*__errno_location() == 4 as libc::c_int) {
                break;
            }
        } else {
            (*ibuf).r.wpos = ((*ibuf).r.wpos).wrapping_add(n as size_t);
            cmsg = if msg.msg_controllen
                >= ::core::mem::size_of::<cmsghdr>() as libc::c_ulong
            {
                msg.msg_control as *mut cmsghdr
            } else {
                0 as *mut cmsghdr
            };
            while !cmsg.is_null() {
                if (*cmsg).cmsg_level == 1 as libc::c_int
                    && (*cmsg).cmsg_type == SCM_RIGHTS as libc::c_int
                {
                    let mut i: libc::c_int = 0;
                    let mut j: libc::c_int = 0;
                    j = ((cmsg as *mut libc::c_char)
                        .offset((*cmsg).cmsg_len as isize)
                        .offset_from(
                            ((*cmsg).__cmsg_data).as_mut_ptr() as *mut libc::c_char,
                        ) as libc::c_long as libc::c_ulong)
                        .wrapping_div(
                            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        ) as libc::c_int;
                    i = 0 as libc::c_int;
                    while i < j {
                        fd = *(((*cmsg).__cmsg_data).as_mut_ptr() as *mut libc::c_int)
                            .offset(i as isize);
                        if !ifd.is_null() {
                            (*ifd).fd = fd;
                            (*ifd).entry.tqe_next = 0 as *mut imsg_fd;
                            (*ifd).entry.tqe_prev = (*ibuf).fds.tqh_last;
                            *(*ibuf).fds.tqh_last = ifd;
                            (*ibuf).fds.tqh_last = &mut (*ifd).entry.tqe_next;
                            ifd = 0 as *mut imsg_fd;
                        } else {
                            close(fd);
                        }
                        i += 1;
                        i;
                    }
                }
                cmsg = __cmsg_nxthdr(&mut msg, cmsg);
            }
            break;
        }
    }
    free(ifd as *mut libc::c_void);
    return n;
}
#[no_mangle]
pub unsafe extern "C" fn imsg_get(
    mut ibuf: *mut imsgbuf,
    mut imsg: *mut imsg,
) -> ssize_t {
    let mut av: size_t = 0;
    let mut left: size_t = 0;
    let mut datalen: size_t = 0;
    av = (*ibuf).r.wpos;
    if ::core::mem::size_of::<imsg_hdr>() as libc::c_ulong > av {
        return 0 as libc::c_int as ssize_t;
    }
    memcpy(
        &mut (*imsg).hdr as *mut imsg_hdr as *mut libc::c_void,
        ((*ibuf).r.buf).as_mut_ptr() as *const libc::c_void,
        ::core::mem::size_of::<imsg_hdr>() as libc::c_ulong,
    );
    if ((*imsg).hdr.len as libc::c_ulong)
        < ::core::mem::size_of::<imsg_hdr>() as libc::c_ulong
        || (*imsg).hdr.len as libc::c_int > 16384 as libc::c_int
    {
        *__errno_location() = 34 as libc::c_int;
        return -(1 as libc::c_int) as ssize_t;
    }
    if (*imsg).hdr.len as size_t > av {
        return 0 as libc::c_int as ssize_t;
    }
    datalen = ((*imsg).hdr.len as libc::c_ulong)
        .wrapping_sub(::core::mem::size_of::<imsg_hdr>() as libc::c_ulong);
    (*ibuf)
        .r
        .rptr = ((*ibuf).r.buf)
        .as_mut_ptr()
        .offset(::core::mem::size_of::<imsg_hdr>() as libc::c_ulong as isize);
    if datalen == 0 as libc::c_int as size_t {
        (*imsg).data = 0 as *mut libc::c_void;
    } else {
        (*imsg).data = malloc(datalen);
        if ((*imsg).data).is_null() {
            return -(1 as libc::c_int) as ssize_t;
        }
    }
    if (*imsg).hdr.flags as libc::c_int & 1 as libc::c_int != 0 {
        (*imsg).fd = imsg_get_fd(ibuf);
    } else {
        (*imsg).fd = -(1 as libc::c_int);
    }
    memcpy((*imsg).data, (*ibuf).r.rptr as *const libc::c_void, datalen);
    if ((*imsg).hdr.len as size_t) < av {
        left = av.wrapping_sub((*imsg).hdr.len as size_t);
        memmove(
            &mut (*ibuf).r.buf as *mut [u_char; 65535] as *mut libc::c_void,
            ((*ibuf).r.buf).as_mut_ptr().offset((*imsg).hdr.len as libc::c_int as isize)
                as *const libc::c_void,
            left,
        );
        (*ibuf).r.wpos = left;
    } else {
        (*ibuf).r.wpos = 0 as libc::c_int as size_t;
    }
    return datalen.wrapping_add(::core::mem::size_of::<imsg_hdr>() as libc::c_ulong)
        as ssize_t;
}
#[no_mangle]
pub unsafe extern "C" fn imsg_compose(
    mut ibuf: *mut imsgbuf,
    mut type_0: uint32_t,
    mut peerid: uint32_t,
    mut pid: pid_t,
    mut fd: libc::c_int,
    mut data: *const libc::c_void,
    mut datalen: uint16_t,
) -> libc::c_int {
    let mut wbuf: *mut ibuf = 0 as *mut ibuf;
    wbuf = imsg_create(ibuf, type_0, peerid, pid, datalen);
    if wbuf.is_null() {
        return -(1 as libc::c_int);
    }
    if imsg_add(wbuf, data, datalen) == -(1 as libc::c_int) {
        return -(1 as libc::c_int);
    }
    (*wbuf).fd = fd;
    imsg_close(ibuf, wbuf);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn imsg_composev(
    mut ibuf: *mut imsgbuf,
    mut type_0: uint32_t,
    mut peerid: uint32_t,
    mut pid: pid_t,
    mut fd: libc::c_int,
    mut iov: *const iovec,
    mut iovcnt: libc::c_int,
) -> libc::c_int {
    let mut wbuf: *mut ibuf = 0 as *mut ibuf;
    let mut i: libc::c_int = 0;
    let mut datalen: libc::c_int = 0 as libc::c_int;
    i = 0 as libc::c_int;
    while i < iovcnt {
        datalen = (datalen as size_t).wrapping_add((*iov.offset(i as isize)).iov_len)
            as libc::c_int as libc::c_int;
        i += 1;
        i;
    }
    wbuf = imsg_create(ibuf, type_0, peerid, pid, datalen as uint16_t);
    if wbuf.is_null() {
        return -(1 as libc::c_int);
    }
    i = 0 as libc::c_int;
    while i < iovcnt {
        if imsg_add(
            wbuf,
            (*iov.offset(i as isize)).iov_base,
            (*iov.offset(i as isize)).iov_len as uint16_t,
        ) == -(1 as libc::c_int)
        {
            return -(1 as libc::c_int);
        }
        i += 1;
        i;
    }
    (*wbuf).fd = fd;
    imsg_close(ibuf, wbuf);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn imsg_create(
    mut ibuf: *mut imsgbuf,
    mut type_0: uint32_t,
    mut peerid: uint32_t,
    mut pid: pid_t,
    mut datalen: uint16_t,
) -> *mut ibuf {
    let mut wbuf: *mut ibuf = 0 as *mut ibuf;
    let mut hdr: imsg_hdr = imsg_hdr {
        type_0: 0,
        len: 0,
        flags: 0,
        peerid: 0,
        pid: 0,
    };
    datalen = (datalen as libc::c_ulong)
        .wrapping_add(::core::mem::size_of::<imsg_hdr>() as libc::c_ulong) as uint16_t
        as uint16_t;
    if datalen as libc::c_int > 16384 as libc::c_int {
        *__errno_location() = 34 as libc::c_int;
        return 0 as *mut ibuf;
    }
    hdr.type_0 = type_0;
    hdr.flags = 0 as libc::c_int as uint16_t;
    hdr.peerid = peerid;
    hdr.pid = pid as uint32_t;
    if hdr.pid == 0 as libc::c_int as uint32_t {
        hdr.pid = (*ibuf).pid as uint32_t;
    }
    wbuf = ibuf_dynamic(datalen as size_t, 16384 as libc::c_int as size_t);
    if wbuf.is_null() {
        return 0 as *mut ibuf;
    }
    if imsg_add(
        wbuf,
        &mut hdr as *mut imsg_hdr as *const libc::c_void,
        ::core::mem::size_of::<imsg_hdr>() as libc::c_ulong as uint16_t,
    ) == -(1 as libc::c_int)
    {
        return 0 as *mut ibuf;
    }
    return wbuf;
}
#[no_mangle]
pub unsafe extern "C" fn imsg_add(
    mut msg: *mut ibuf,
    mut data: *const libc::c_void,
    mut datalen: uint16_t,
) -> libc::c_int {
    if datalen != 0 {
        if ibuf_add(msg, data, datalen as size_t) == -(1 as libc::c_int) {
            ibuf_free(msg);
            return -(1 as libc::c_int);
        }
    }
    return datalen as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn imsg_close(mut ibuf: *mut imsgbuf, mut msg: *mut ibuf) {
    let mut hdr: *mut imsg_hdr = 0 as *mut imsg_hdr;
    hdr = (*msg).buf as *mut imsg_hdr;
    (*hdr).flags = ((*hdr).flags as libc::c_int & !(1 as libc::c_int)) as uint16_t;
    if (*msg).fd != -(1 as libc::c_int) {
        (*hdr).flags = ((*hdr).flags as libc::c_int | 1 as libc::c_int) as uint16_t;
    }
    (*hdr).len = (*msg).wpos as uint16_t;
    ibuf_close(&mut (*ibuf).w, msg);
}
#[no_mangle]
pub unsafe extern "C" fn imsg_free(mut imsg: *mut imsg) {
    freezero(
        (*imsg).data,
        ((*imsg).hdr.len as libc::c_ulong)
            .wrapping_sub(::core::mem::size_of::<imsg_hdr>() as libc::c_ulong),
    );
}
unsafe extern "C" fn imsg_get_fd(mut ibuf: *mut imsgbuf) -> libc::c_int {
    let mut fd: libc::c_int = 0;
    let mut ifd: *mut imsg_fd = 0 as *mut imsg_fd;
    ifd = (*ibuf).fds.tqh_first;
    if ifd.is_null() {
        return -(1 as libc::c_int);
    }
    fd = (*ifd).fd;
    if !((*ifd).entry.tqe_next).is_null() {
        (*(*ifd).entry.tqe_next).entry.tqe_prev = (*ifd).entry.tqe_prev;
    } else {
        (*ibuf).fds.tqh_last = (*ifd).entry.tqe_prev;
    }
    *(*ifd).entry.tqe_prev = (*ifd).entry.tqe_next;
    free(ifd as *mut libc::c_void);
    return fd;
}
#[no_mangle]
pub unsafe extern "C" fn imsg_flush(mut ibuf: *mut imsgbuf) -> libc::c_int {
    while (*ibuf).w.queued != 0 {
        if msgbuf_write(&mut (*ibuf).w) <= 0 as libc::c_int {
            return -(1 as libc::c_int);
        }
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn imsg_clear(mut ibuf: *mut imsgbuf) {
    let mut fd: libc::c_int = 0;
    msgbuf_clear(&mut (*ibuf).w);
    loop {
        fd = imsg_get_fd(ibuf);
        if !(fd != -(1 as libc::c_int)) {
            break;
        }
        close(fd);
    };
}
