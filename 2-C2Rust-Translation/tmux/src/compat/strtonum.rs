use ::libc;
extern "C" {
    fn __errno_location() -> *mut libc::c_int;
    fn strtoll(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_longlong;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct errval {
    pub errstr: *const libc::c_char,
    pub err: libc::c_int,
}
#[no_mangle]
pub unsafe extern "C" fn strtonum(
    mut numstr: *const libc::c_char,
    mut minval: libc::c_longlong,
    mut maxval: libc::c_longlong,
    mut errstrp: *mut *const libc::c_char,
) -> libc::c_longlong {
    let mut ll: libc::c_longlong = 0 as libc::c_int as libc::c_longlong;
    let mut ep: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut error: libc::c_int = 0 as libc::c_int;
    let mut ev: [errval; 4] = [
        {
            let mut init = errval {
                errstr: 0 as *const libc::c_char,
                err: 0 as libc::c_int,
            };
            init
        },
        {
            let mut init = errval {
                errstr: b"invalid\0" as *const u8 as *const libc::c_char,
                err: 22 as libc::c_int,
            };
            init
        },
        {
            let mut init = errval {
                errstr: b"too small\0" as *const u8 as *const libc::c_char,
                err: 34 as libc::c_int,
            };
            init
        },
        {
            let mut init = errval {
                errstr: b"too large\0" as *const u8 as *const libc::c_char,
                err: 34 as libc::c_int,
            };
            init
        },
    ];
    ev[0 as libc::c_int as usize].err = *__errno_location();
    *__errno_location() = 0 as libc::c_int;
    if minval > maxval {
        error = 1 as libc::c_int;
    } else {
        ll = strtoll(numstr, &mut ep, 10 as libc::c_int);
        if numstr == ep as *const libc::c_char || *ep as libc::c_int != '\0' as i32 {
            error = 1 as libc::c_int;
        } else if ll
            == -(9223372036854775807 as libc::c_longlong) - 1 as libc::c_longlong
            && *__errno_location() == 34 as libc::c_int || ll < minval
        {
            error = 2 as libc::c_int;
        } else if ll == 9223372036854775807 as libc::c_longlong
            && *__errno_location() == 34 as libc::c_int || ll > maxval
        {
            error = 3 as libc::c_int;
        }
    }
    if !errstrp.is_null() {
        *errstrp = ev[error as usize].errstr;
    }
    *__errno_location() = ev[error as usize].err;
    if error != 0 {
        ll = 0 as libc::c_int as libc::c_longlong;
    }
    return ll;
}
