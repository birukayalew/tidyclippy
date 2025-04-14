use ::libc;
extern "C" {
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn freezero(mut ptr: *mut libc::c_void, mut size: size_t) {
    if !ptr.is_null() {
        memset(ptr, 0 as libc::c_int, size);
        free(ptr);
    }
}
