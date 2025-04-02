use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
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
    fn explicit_bzero(__s: *mut libc::c_void, __n: size_t);
    fn getpagesize() -> libc::c_int;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn recallocarray(
    mut ptr: *mut libc::c_void,
    mut oldnmemb: size_t,
    mut newnmemb: size_t,
    mut size: size_t,
) -> *mut libc::c_void {
    let mut oldsize: size_t = 0;
    let mut newsize: size_t = 0;
    let mut newptr: *mut libc::c_void = 0 as *mut libc::c_void;
    if ptr.is_null() {
        return calloc(newnmemb, size);
    }
    if (newnmemb
        >= (1 as libc::c_int as size_t)
            << (::core::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_mul(4 as libc::c_int as libc::c_ulong)
        || size
            >= (1 as libc::c_int as size_t)
                << (::core::mem::size_of::<size_t>() as libc::c_ulong)
                    .wrapping_mul(4 as libc::c_int as libc::c_ulong))
        && newnmemb > 0 as libc::c_int as size_t
        && (18446744073709551615 as libc::c_ulong).wrapping_div(newnmemb) < size
    {
        *__errno_location() = 12 as libc::c_int;
        return 0 as *mut libc::c_void;
    }
    newsize = newnmemb * size;
    if (oldnmemb
        >= (1 as libc::c_int as size_t)
            << (::core::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_mul(4 as libc::c_int as libc::c_ulong)
        || size
            >= (1 as libc::c_int as size_t)
                << (::core::mem::size_of::<size_t>() as libc::c_ulong)
                    .wrapping_mul(4 as libc::c_int as libc::c_ulong))
        && oldnmemb > 0 as libc::c_int as size_t
        && (18446744073709551615 as libc::c_ulong).wrapping_div(oldnmemb) < size
    {
        *__errno_location() = 22 as libc::c_int;
        return 0 as *mut libc::c_void;
    }
    oldsize = oldnmemb * size;
    if newsize <= oldsize {
        let mut d: size_t = oldsize.wrapping_sub(newsize);
        if d < oldsize / 2 as libc::c_int as size_t && d < getpagesize() as size_t {
            memset(
                (ptr as *mut libc::c_char).offset(newsize as isize) as *mut libc::c_void,
                0 as libc::c_int,
                d,
            );
            return ptr;
        }
    }
    newptr = malloc(newsize);
    if newptr.is_null() {
        return 0 as *mut libc::c_void;
    }
    if newsize > oldsize {
        memcpy(newptr, ptr, oldsize);
        memset(
            (newptr as *mut libc::c_char).offset(oldsize as isize) as *mut libc::c_void,
            0 as libc::c_int,
            newsize.wrapping_sub(oldsize),
        );
    } else {
        memcpy(newptr, ptr, newsize);
    }
    explicit_bzero(ptr, oldsize);
    free(ptr);
    return newptr;
}
