use ::libc;
extern "C" {
    fn vsnprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ::core::ffi::VaList,
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
pub type size_t = libc::c_ulong;
pub type opng_ullong_t = libc::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_ulratio {
    pub num: libc::c_ulong,
    pub denom: libc::c_ulong,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct opng_ullratio {
    pub num: opng_ullong_t,
    pub denom: opng_ullong_t,
}
pub type opng_uxlong_impl_t = opng_ullong_t;
pub type va_list = __builtin_va_list;
unsafe extern "C" fn opng_snprintf_impl(
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
    mut format: *const libc::c_char,
    mut args: ...
) -> libc::c_int {
    let mut arg_ptr: ::core::ffi::VaListImpl;
    let mut result: libc::c_int = 0;
    arg_ptr = args.clone();
    result = vsnprintf(buffer, buffer_size, format, arg_ptr.as_va_list());
    if result < 0 as libc::c_int || result as size_t >= buffer_size {
        if buffer_size > 0 as libc::c_int as size_t {
            *buffer
                .offset(
                    buffer_size.wrapping_sub(1 as libc::c_int as size_t) as isize,
                ) = '\0' as i32 as libc::c_char;
        }
        return -(1 as libc::c_int);
    }
    return result;
}
unsafe extern "C" fn opng_sprint_uratio_impl(
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
    mut num: opng_uxlong_impl_t,
    mut denom: opng_uxlong_impl_t,
    mut always_percent: libc::c_int,
) -> libc::c_int {
    let mut integer_part: opng_uxlong_impl_t = 0;
    let mut remainder: opng_uxlong_impl_t = 0;
    let mut fractional_part: libc::c_uint = 0;
    let mut scale: libc::c_uint = 0;
    let mut scaled_ratio: libc::c_double = 0.;
    if denom == 0 as libc::c_int as opng_uxlong_impl_t {
        return opng_snprintf_impl(
            buffer,
            buffer_size,
            if num == 0 as libc::c_int as opng_uxlong_impl_t {
                b"??%%\0" as *const u8 as *const libc::c_char
            } else {
                b"INFINITY%%\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if num < denom
        && denom / denom.wrapping_sub(num) < 20000 as libc::c_int as opng_uxlong_impl_t
    {
        scale = 10000 as libc::c_int as libc::c_uint;
        scaled_ratio = num as libc::c_double * scale as libc::c_double
            / denom as libc::c_double;
        fractional_part = (scaled_ratio + 0.5f64) as libc::c_uint;
        if fractional_part >= scale {
            fractional_part = scale.wrapping_sub(1 as libc::c_int as libc::c_uint);
        }
        return opng_snprintf_impl(
            buffer,
            buffer_size,
            b"%u.%02u%%\0" as *const u8 as *const libc::c_char,
            fractional_part.wrapping_div(100 as libc::c_int as libc::c_uint),
            fractional_part.wrapping_rem(100 as libc::c_int as libc::c_uint),
        );
    }
    integer_part = num / denom;
    remainder = num % denom;
    scale = 100 as libc::c_int as libc::c_uint;
    scaled_ratio = remainder as libc::c_double * scale as libc::c_double
        / denom as libc::c_double;
    fractional_part = (scaled_ratio + 0.5f64) as libc::c_uint;
    if fractional_part >= scale {
        fractional_part = 0 as libc::c_int as libc::c_uint;
        integer_part = integer_part.wrapping_add(1);
        integer_part;
    }
    if always_percent != 0 {
        return opng_snprintf_impl(
            buffer,
            buffer_size,
            b"%llu%02u%%\0" as *const u8 as *const libc::c_char,
            integer_part,
            fractional_part,
        );
    }
    if integer_part < 100 as libc::c_int as opng_uxlong_impl_t {
        return opng_snprintf_impl(
            buffer,
            buffer_size,
            b"%llu.%02ux\0" as *const u8 as *const libc::c_char,
            integer_part,
            fractional_part,
        );
    }
    integer_part = num / denom;
    if remainder
        > denom.wrapping_sub(1 as libc::c_int as opng_uxlong_impl_t)
            / 2 as libc::c_int as opng_uxlong_impl_t
    {
        integer_part = integer_part.wrapping_add(1);
        integer_part;
    }
    return opng_snprintf_impl(
        buffer,
        buffer_size,
        b"%llux\0" as *const u8 as *const libc::c_char,
        integer_part,
    );
}
#[no_mangle]
pub unsafe extern "C" fn opng_ulratio_to_factor_string(
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
    mut ratio: *const opng_ulratio,
) -> libc::c_int {
    let mut num: opng_uxlong_impl_t = (*ratio).num as opng_uxlong_impl_t;
    let mut denom: opng_uxlong_impl_t = (*ratio).denom as opng_uxlong_impl_t;
    return opng_sprint_uratio_impl(buffer, buffer_size, num, denom, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn opng_ulratio_to_percent_string(
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
    mut ratio: *const opng_ulratio,
) -> libc::c_int {
    let mut num: opng_uxlong_impl_t = (*ratio).num as opng_uxlong_impl_t;
    let mut denom: opng_uxlong_impl_t = (*ratio).denom as opng_uxlong_impl_t;
    return opng_sprint_uratio_impl(buffer, buffer_size, num, denom, 1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn opng_ullratio_to_factor_string(
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
    mut ratio: *const opng_ullratio,
) -> libc::c_int {
    let mut num: opng_uxlong_impl_t = (*ratio).num;
    let mut denom: opng_uxlong_impl_t = (*ratio).denom;
    return opng_sprint_uratio_impl(buffer, buffer_size, num, denom, 0 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn opng_ullratio_to_percent_string(
    mut buffer: *mut libc::c_char,
    mut buffer_size: size_t,
    mut ratio: *const opng_ullratio,
) -> libc::c_int {
    let mut num: opng_uxlong_impl_t = (*ratio).num;
    let mut denom: opng_uxlong_impl_t = (*ratio).denom;
    return opng_sprint_uratio_impl(buffer, buffer_size, num, denom, 1 as libc::c_int);
}
