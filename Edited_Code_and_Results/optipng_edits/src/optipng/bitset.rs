use ::libc;
extern "C" {
    fn isspace(_: libc::c_int) -> libc::c_int;
    fn __errno_location() -> *mut libc::c_int;
}
pub type opng_bitset_t = libc::c_uint;
pub type C2RustUnnamed = libc::c_uint;
pub const OPNG_BITSET_ELT_MAX: C2RustUnnamed = 31;
pub const OPNG_BITSET_ELT_MIN: C2RustUnnamed = 0;
#[no_mangle]
pub unsafe extern "C" fn opng_bitset_count(mut set: opng_bitset_t) -> libc::c_uint {
    let mut result: libc::c_uint = 0;
    result = 0 as libc::c_int as libc::c_uint;
    while set != 0 as libc::c_int as opng_bitset_t {
        set &= set.wrapping_sub(1 as libc::c_int as opng_bitset_t);
        result = result.wrapping_add(1);
        result;
    }
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn opng_bitset_find_first(mut set: opng_bitset_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i <= OPNG_BITSET_ELT_MAX as libc::c_int {
        if set & (1 as libc::c_uint) << i != 0 as libc::c_int as libc::c_uint {
            return i;
        }
        i += 1;
        i;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn opng_bitset_find_next(
    mut set: opng_bitset_t,
    mut elt: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = (if elt > -(1 as libc::c_int) { elt } else { -(1 as libc::c_int) })
        + 1 as libc::c_int;
    while i <= OPNG_BITSET_ELT_MAX as libc::c_int {
        if set & (1 as libc::c_uint) << i != 0 as libc::c_int as libc::c_uint {
            return i;
        }
        i += 1;
        i;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn opng_bitset_find_last(mut set: opng_bitset_t) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = OPNG_BITSET_ELT_MAX as libc::c_int;
    while i >= 0 as libc::c_int {
        if set & (1 as libc::c_uint) << i != 0 as libc::c_int as libc::c_uint {
            return i;
        }
        i -= 1;
        i;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn opng_bitset_find_prev(
    mut set: opng_bitset_t,
    mut elt: libc::c_int,
) -> libc::c_int {
    let mut i: libc::c_int = 0;
    i = (if elt < OPNG_BITSET_ELT_MAX as libc::c_int + 1 as libc::c_int {
        elt
    } else {
        OPNG_BITSET_ELT_MAX as libc::c_int + 1 as libc::c_int
    }) - 1 as libc::c_int;
    while i >= 0 as libc::c_int {
        if set & (1 as libc::c_uint) << i != 0 as libc::c_int as libc::c_uint {
            return i;
        }
        i -= 1;
        i;
    }
    return -(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn opng_strparse_rangeset_to_bitset(
    mut out_set: *mut opng_bitset_t,
    mut rangeset_str: *const libc::c_char,
    mut mask_set: opng_bitset_t,
) -> libc::c_int {
    let mut result: opng_bitset_t = 0;
    let mut ptr: *const libc::c_char = 0 as *const libc::c_char;
    let mut state: libc::c_int = 0;
    let mut num: libc::c_int = 0;
    let mut num1: libc::c_int = 0;
    let mut num2: libc::c_int = 0;
    let mut err_invalid: libc::c_int = 0;
    let mut err_range: libc::c_int = 0;
    result = 0 as libc::c_uint;
    ptr = rangeset_str;
    state = 0 as libc::c_int;
    err_range = 0 as libc::c_int;
    err_invalid = err_range;
    num2 = -(1 as libc::c_int);
    num1 = num2;
    let mut current_block_37: u64;
    loop {
        while isspace(*ptr as libc::c_int) != 0 {
            ptr = ptr.offset(1);
            ptr;
        }
        match state {
            0 => {
                current_block_37 = 4631248241652996018;
            }
            2 => {
                current_block_37 = 4631248241652996018;
            }
            1 => {
                if *ptr as libc::c_int == '-' as i32 {
                    ptr = ptr.offset(1);
                    ptr;
                    num2 = OPNG_BITSET_ELT_MAX as libc::c_int;
                    state += 1;
                    state;
                    continue;
                } else {
                    current_block_37 = 15345278821338558188;
                }
            }
            _ => {
                current_block_37 = 15345278821338558188;
            }
        }
        match current_block_37 {
            4631248241652996018 => {
                if *ptr as libc::c_int >= '0' as i32 && *ptr as libc::c_int <= '9' as i32
                {
                    num = 0 as libc::c_int;
                    loop {
                        num = 10 as libc::c_int * num
                            + (*ptr as libc::c_int - '0' as i32);
                        if num > OPNG_BITSET_ELT_MAX as libc::c_int {
                            num = OPNG_BITSET_ELT_MAX as libc::c_int;
                            err_range = 1 as libc::c_int;
                        }
                        ptr = ptr.offset(1);
                        ptr;
                        if !(*ptr as libc::c_int >= '0' as i32
                            && *ptr as libc::c_int <= '9' as i32)
                        {
                            break;
                        }
                    }
                    if !(mask_set & (1 as libc::c_uint) << num
                        != 0 as libc::c_int as libc::c_uint)
                    {
                        err_range = 1 as libc::c_int;
                    }
                    if state == 0 as libc::c_int {
                        num1 = num;
                    }
                    num2 = num;
                    state += 1;
                    state;
                    continue;
                }
            }
            _ => {}
        }
        if state > 0 as libc::c_int {
            if num1 <= num2 {
                result
                    |= if num1 <= num2 {
                        ((1 as libc::c_uint) << num2 - num1 << 1 as libc::c_int)
                            .wrapping_sub(1 as libc::c_int as libc::c_uint) << num1
                    } else {
                        0 as libc::c_uint
                    };
                result &= mask_set;
            } else {
                err_range = 1 as libc::c_int;
            }
            state = 0 as libc::c_int;
        }
        if *ptr as libc::c_int == ',' as i32 || *ptr as libc::c_int == ';' as i32 {
            ptr = ptr.offset(1);
            ptr;
        } else {
            if !(*ptr as libc::c_int == '-' as i32) {
                break;
            }
            err_invalid = 1 as libc::c_int;
            break;
        }
    }
    while isspace(*ptr as libc::c_int) != 0 {
        ptr = ptr.offset(1);
        ptr;
    }
    if *ptr as libc::c_int != '\0' as i32 {
        err_invalid = 1 as libc::c_int;
    }
    if err_invalid != 0 {
        *__errno_location() = 22 as libc::c_int;
        *out_set = 0 as libc::c_uint;
        return -(1 as libc::c_int);
    } else if err_range != 0 {
        *__errno_location() = 34 as libc::c_int;
        *out_set = !(0 as libc::c_uint);
        return -(1 as libc::c_int);
    } else {
        *out_set = result;
        return 0 as libc::c_int;
    };
}
