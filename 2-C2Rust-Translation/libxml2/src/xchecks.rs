use ::libc;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlNodeCommon {
    pub ptr1: *mut libc::c_void,
    pub type_0: libc::c_uint,
}
#[no_mangle]
pub unsafe extern "C" fn __c2rust_hash_internal_state_struct(
    mut x: *mut libc::c_void,
    mut depth: size_t,
) -> uint64_t {
    return 0xabcd0001 as libc::c_uint as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn __c2rust_hash_lzma_internal_s_struct(
    mut x: *mut libc::c_void,
    mut depth: size_t,
) -> uint64_t {
    return 0xabcd0002 as libc::c_uint as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn xcheck_hash_xmlSchemaValDate(
    mut x: *mut libc::c_void,
    mut depth: size_t,
) -> uint64_t {
    return 0xabcd0003 as libc::c_uint as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn xcheck_hash_xmlSchemaValDecimal(
    mut x: *mut libc::c_void,
    mut depth: size_t,
) -> uint64_t {
    return 0xabcd0004 as libc::c_uint as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn xcheck_hash_xmlSchemaType(
    mut x: *mut libc::c_void,
    mut depth: size_t,
) -> uint64_t {
    return ((0xabcd0005 as libc::c_ulonglong) << 32 as libc::c_int
        | *(x as *mut uint32_t) as libc::c_ulonglong) as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn xcheck_hash_xmlSchemaAttribute(
    mut x: *mut libc::c_void,
    mut depth: size_t,
) -> uint64_t {
    return ((0xabcd0005 as libc::c_ulonglong) << 32 as libc::c_int
        | *(x as *mut uint32_t) as libc::c_ulonglong) as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn xcheck_hash_xmlSchemaWildcard(
    mut x: *mut libc::c_void,
    mut depth: size_t,
) -> uint64_t {
    return ((0xabcd0005 as libc::c_ulonglong) << 32 as libc::c_int
        | *(x as *mut uint32_t) as libc::c_ulonglong) as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn xcheck_hash_xmlSchemaAttributeGroup(
    mut x: *mut libc::c_void,
    mut depth: size_t,
) -> uint64_t {
    return ((0xabcd0005 as libc::c_ulonglong) << 32 as libc::c_int
        | *(x as *mut uint32_t) as libc::c_ulonglong) as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn xcheck_hash_xmlSchemaElement(
    mut x: *mut libc::c_void,
    mut depth: size_t,
) -> uint64_t {
    return ((0xabcd0005 as libc::c_ulonglong) << 32 as libc::c_int
        | *(x as *mut uint32_t) as libc::c_ulonglong) as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn xcheck_hash_xmlSchemaFacet(
    mut x: *mut libc::c_void,
    mut depth: size_t,
) -> uint64_t {
    return ((0xabcd0005 as libc::c_ulonglong) << 32 as libc::c_int
        | *(x as *mut uint32_t) as libc::c_ulonglong) as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn xcheck_hash_xmlSchemaNotation(
    mut x: *mut libc::c_void,
    mut depth: size_t,
) -> uint64_t {
    return ((0xabcd0005 as libc::c_ulonglong) << 32 as libc::c_int
        | *(x as *mut uint32_t) as libc::c_ulonglong) as uint64_t;
}
#[no_mangle]
pub unsafe extern "C" fn xcheck_hash_xmlNode(
    mut x: *mut libc::c_void,
    mut depth: size_t,
) -> uint64_t {
    let mut nc: *mut _xmlNodeCommon = x as *mut _xmlNodeCommon;
    return ((0xabcd0006 as libc::c_ulonglong) << 32 as libc::c_int
        | (*nc).type_0 as libc::c_ulonglong) as uint64_t;
}
