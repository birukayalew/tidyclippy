use ::libc;
pub type __uint64_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
#[no_mangle]
pub unsafe extern "C" fn __c2rust_hash_ldat_struct(
    mut l: *mut libc::c_void,
    mut depth: size_t,
) -> uint64_t {
    return 0xabcd0001 as libc::c_uint as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn __c2rust_hash_dst_key_struct(
    mut l: *mut libc::c_void,
    mut depth: size_t,
) -> uint64_t {
    return 0xabcd0002 as libc::c_uint as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn __c2rust_hash_evbuffer_struct(
    mut l: *mut libc::c_void,
    mut depth: size_t,
) -> uint64_t {
    return 0xabcd0003 as libc::c_uint as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn __c2rust_hash___locale_data_struct(
    mut l: *mut libc::c_void,
    mut depth: size_t,
) -> uint64_t {
    return 0xabcd0004 as libc::c_uint as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn __c2rust_hash_event_base_struct(
    mut l: *mut libc::c_void,
    mut depth: size_t,
) -> uint64_t {
    return 0xabcd0005 as libc::c_uint as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn __c2rust_hash___mbstate_t_struct(
    mut l: *mut libc::c_void,
    mut depth: size_t,
) -> uint64_t {
    return 0xabcd0006 as libc::c_uint as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn __c2rust_hash_bufferevent_ops_struct(
    mut l: *mut libc::c_void,
    mut depth: size_t,
) -> uint64_t {
    return 0xabcd0007 as libc::c_uint as uint64_t;
}
