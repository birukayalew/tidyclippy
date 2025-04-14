use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type C2RustUnnamed = libc::c_uint;
pub const PNM_P7: C2RustUnnamed = 7;
pub const PNM_P6: C2RustUnnamed = 6;
pub const PNM_P5: C2RustUnnamed = 5;
pub const PNM_P4: C2RustUnnamed = 4;
pub const PNM_P3: C2RustUnnamed = 3;
pub const PNM_P2: C2RustUnnamed = 2;
pub const PNM_P1: C2RustUnnamed = 1;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct pnm_struct {
    pub format: libc::c_uint,
    pub depth: libc::c_uint,
    pub width: libc::c_uint,
    pub height: libc::c_uint,
    pub maxval: libc::c_uint,
}
#[no_mangle]
pub unsafe extern "C" fn pnm_is_valid(mut pnm_ptr: *const pnm_struct) -> libc::c_int {
    let mut format: libc::c_uint = (*pnm_ptr).format;
    let mut depth: libc::c_uint = (*pnm_ptr).depth;
    let mut width: libc::c_uint = (*pnm_ptr).width;
    let mut height: libc::c_uint = (*pnm_ptr).height;
    let mut maxval: libc::c_uint = (*pnm_ptr).maxval;
    if depth == 0 as libc::c_int as libc::c_uint
        || width == 0 as libc::c_int as libc::c_uint
        || height == 0 as libc::c_int as libc::c_uint
        || maxval == 0 as libc::c_int as libc::c_uint
    {
        return 0 as libc::c_int;
    }
    match format {
        1 | 4 => {
            return if depth == 1 as libc::c_int as libc::c_uint
                && maxval == 1 as libc::c_int as libc::c_uint
            {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            };
        }
        2 | 5 => {
            return if depth == 1 as libc::c_int as libc::c_uint {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            };
        }
        3 | 6 => {
            return if depth == 3 as libc::c_int as libc::c_uint {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            };
        }
        7 => return 1 as libc::c_int,
        _ => return 0 as libc::c_int,
    };
}
#[no_mangle]
pub unsafe extern "C" fn pnm_raw_sample_size(mut pnm_ptr: *const pnm_struct) -> size_t {
    let mut maxval: libc::c_uint = (*pnm_ptr).maxval;
    if maxval == 0 as libc::c_int as libc::c_uint {
        *__errno_location() = 22 as libc::c_int;
    }
    if maxval <= 0xff as libc::c_uint {
        return 1 as libc::c_int as size_t
    } else if maxval <= 0xffff as libc::c_uint {
        return 2 as libc::c_int as size_t
    } else if maxval <= 0xffffff as libc::c_uint {
        return 3 as libc::c_int as size_t
    } else if maxval <= 0xffffffff as libc::c_uint {
        return 4 as libc::c_int as size_t
    } else {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int as size_t;
    };
}
#[no_mangle]
pub unsafe extern "C" fn pnm_mem_size(
    mut pnm_ptr: *const pnm_struct,
    mut sample_size: size_t,
    mut num_rows: libc::c_uint,
) -> size_t {
    let mut depth: libc::c_uint = (*pnm_ptr).depth;
    let mut width: libc::c_uint = (*pnm_ptr).width;
    if sample_size == 0 as libc::c_int as size_t
        || depth == 0 as libc::c_int as libc::c_uint
        || width == 0 as libc::c_int as libc::c_uint
    {
        *__errno_location() = 22 as libc::c_int;
        return 0 as libc::c_int as size_t;
    }
    if num_rows as size_t
        > -(1 as libc::c_int) as size_t / sample_size / depth as size_t / width as size_t
    {
        *__errno_location() = 34 as libc::c_int;
        return 0 as libc::c_int as size_t;
    }
    return sample_size * depth as size_t * width as size_t * num_rows as size_t;
}
