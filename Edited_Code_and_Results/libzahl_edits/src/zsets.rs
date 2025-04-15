#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#![feature(asm, label_break_value)]
use core::arch::asm;
extern "C" {
    fn libzahl_realloc(_: *mut zahl, _: size_t);
    fn zadd(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn zmul_ll(_: *mut zahl, _: *mut zahl, _: *mut zahl);
    fn __errno_location() -> *mut libc::c_int;
    static mut libzahl_tmp_str_num: z_t;
    static mut libzahl_const_1e19: z_t;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
}
pub type size_t = libc::c_ulong;
pub type __uint64_t = libc::c_ulong;
pub type uint64_t = __uint64_t;
pub type zahl_char_t = uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct zahl {
    pub sign: libc::c_int,
    pub padding__: libc::c_int,
    pub used: size_t,
    pub alloced: size_t,
    pub chars: *mut zahl_char_t,
}
pub type z_t = [zahl; 1];
pub const _ISdigit: C2RustUnnamed = 2048;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
#[inline]
unsafe extern "C" fn libzahl_memcpy(
    mut d: *mut zahl_char_t,
    mut s: *const zahl_char_t,
    mut n: size_t,
) {
    let mut current_block_42: u64;
    match n {
        20 => {
            *d
                .offset(
                    (20 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((20 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 17123592612869879796;
        }
        19 => {
            current_block_42 = 17123592612869879796;
        }
        18 => {
            current_block_42 = 7295796347073542504;
        }
        17 => {
            current_block_42 = 1678841039753865640;
        }
        16 => {
            current_block_42 = 4428433140869529006;
        }
        15 => {
            current_block_42 = 7060192243506437934;
        }
        14 => {
            current_block_42 = 10319869968961460719;
        }
        13 => {
            current_block_42 = 1232922633126777150;
        }
        12 => {
            current_block_42 = 16818959577490016211;
        }
        11 => {
            current_block_42 = 11360923742995600144;
        }
        10 => {
            current_block_42 = 12310883806328202904;
        }
        9 => {
            current_block_42 = 6807571309547269490;
        }
        8 => {
            current_block_42 = 2540506661003364611;
        }
        7 => {
            current_block_42 = 13253931237392700050;
        }
        6 => {
            current_block_42 = 6589414636452177323;
        }
        5 => {
            current_block_42 = 1198327748524965992;
        }
        4 => {
            current_block_42 = 7339321152356695781;
        }
        3 => {
            current_block_42 = 8085337086024616189;
        }
        2 => {
            current_block_42 = 8830848799650308304;
        }
        1 => {
            current_block_42 = 11295390336478990698;
        }
        0 => {
            current_block_42 = 1836292691772056875;
        }
        _ => {
            let mut t: zahl_char_t = 0;
            asm!(
                "\n    shlq $3, {3}\n    addq {1}, {3}\n 1:\n    movq 0({2}), {0}\n    movq {0}, 0({1})\n    movq 8({2}), {0}\n    movq {0}, 8({1})\n    movq 16({2}), {0}\n    movq {0}, 16({1})\n    movq 24({2}), {0}\n    movq {0}, 24({1})\n    addq $32, {2}\n    addq $32, {1}\n    cmpq {3}, {1}\n    jl 1b",
                lateout(reg) t, inlateout(reg) d, inlateout(reg) s, inlateout(reg) n,
                options(preserves_flags, att_syntax)
            );
            current_block_42 = 1836292691772056875;
        }
    }
    match current_block_42 {
        17123592612869879796 => {
            *d
                .offset(
                    (19 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((19 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 7295796347073542504;
        }
        _ => {}
    }
    match current_block_42 {
        7295796347073542504 => {
            *d
                .offset(
                    (18 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((18 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 1678841039753865640;
        }
        _ => {}
    }
    match current_block_42 {
        1678841039753865640 => {
            *d
                .offset(
                    (17 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((17 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 4428433140869529006;
        }
        _ => {}
    }
    match current_block_42 {
        4428433140869529006 => {
            *d
                .offset(
                    (16 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((16 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 7060192243506437934;
        }
        _ => {}
    }
    match current_block_42 {
        7060192243506437934 => {
            *d
                .offset(
                    (15 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((15 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 10319869968961460719;
        }
        _ => {}
    }
    match current_block_42 {
        10319869968961460719 => {
            *d
                .offset(
                    (14 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((14 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 1232922633126777150;
        }
        _ => {}
    }
    match current_block_42 {
        1232922633126777150 => {
            *d
                .offset(
                    (13 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((13 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 16818959577490016211;
        }
        _ => {}
    }
    match current_block_42 {
        16818959577490016211 => {
            *d
                .offset(
                    (12 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((12 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 11360923742995600144;
        }
        _ => {}
    }
    match current_block_42 {
        11360923742995600144 => {
            *d
                .offset(
                    (11 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((11 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 12310883806328202904;
        }
        _ => {}
    }
    match current_block_42 {
        12310883806328202904 => {
            *d
                .offset(
                    (10 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((10 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 6807571309547269490;
        }
        _ => {}
    }
    match current_block_42 {
        6807571309547269490 => {
            *d
                .offset(
                    (9 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((9 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 2540506661003364611;
        }
        _ => {}
    }
    match current_block_42 {
        2540506661003364611 => {
            *d
                .offset(
                    (8 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((8 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 13253931237392700050;
        }
        _ => {}
    }
    match current_block_42 {
        13253931237392700050 => {
            *d
                .offset(
                    (7 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((7 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 6589414636452177323;
        }
        _ => {}
    }
    match current_block_42 {
        6589414636452177323 => {
            *d
                .offset(
                    (6 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((6 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 1198327748524965992;
        }
        _ => {}
    }
    match current_block_42 {
        1198327748524965992 => {
            *d
                .offset(
                    (5 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((5 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 7339321152356695781;
        }
        _ => {}
    }
    match current_block_42 {
        7339321152356695781 => {
            *d
                .offset(
                    (4 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((4 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 8085337086024616189;
        }
        _ => {}
    }
    match current_block_42 {
        8085337086024616189 => {
            *d
                .offset(
                    (3 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((3 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 8830848799650308304;
        }
        _ => {}
    }
    match current_block_42 {
        8830848799650308304 => {
            *d
                .offset(
                    (2 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((2 as libc::c_int - 1 as libc::c_int) as isize);
            current_block_42 = 11295390336478990698;
        }
        _ => {}
    }
    match current_block_42 {
        11295390336478990698 => {
            *d
                .offset(
                    (11 as libc::c_int - 1 as libc::c_int) as isize,
                ) = *s.offset((11 as libc::c_int - 1 as libc::c_int) as isize);
        }
        _ => {}
    };
}
#[inline]
unsafe extern "C" fn zsignum(mut a: *mut zahl) -> libc::c_int {
    return (*a).sign;
}
#[inline]
unsafe extern "C" fn zset(mut a: *mut zahl, mut b: *mut zahl) {
    if ((*b).sign == 0 as libc::c_int) as libc::c_int as libc::c_long != 0 {
        (*a).sign = 0 as libc::c_int;
    } else {
        (*a).sign = (*b).sign;
        (*a).used = (*b).used;
        if (*a).alloced < (*b).used {
            libzahl_realloc(a, (*b).used);
        }
        libzahl_memcpy((*a).chars, (*b).chars, (*b).used);
    };
}
#[inline]
unsafe extern "C" fn zmul(mut a: *mut zahl, mut b: *mut zahl, mut c: *mut zahl) {
    let mut b_sign: libc::c_int = 0;
    let mut c_sign: libc::c_int = 0;
    b_sign = (*b).sign;
    (*b).sign *= b_sign;
    c_sign = (*c).sign;
    (*c).sign *= c_sign;
    zmul_ll(a, b, c);
    (*c).sign = c_sign;
    (*b).sign = b_sign;
    (*a).sign = zsignum(b) * zsignum(c);
}
#[no_mangle]
pub unsafe extern "C" fn zsets(
    mut a: *mut zahl,
    mut str: *const libc::c_char,
) -> libc::c_int {
    let mut temp: libc::c_ulonglong = 0 as libc::c_int as libc::c_ulonglong;
    let mut neg: libc::c_int = (*str as libc::c_int == '-' as i32) as libc::c_int;
    let mut str_end: *const libc::c_char = 0 as *const libc::c_char;
    str = str
        .offset((neg != 0 || *str as libc::c_int == '+' as i32) as libc::c_int as isize);
    if (*str == 0) as libc::c_int as libc::c_long != 0 {
        *__errno_location() = 22 as libc::c_int;
        return -(1 as libc::c_int);
    }
    str_end = str;
    while *str_end != 0 {
        if (*(*__ctype_b_loc()).offset(*str_end as libc::c_int as isize) as libc::c_int
            & _ISdigit as libc::c_int as libc::c_ushort as libc::c_int == 0)
            as libc::c_int as libc::c_long != 0
        {
            *__errno_location() = 22 as libc::c_int;
            return -(1 as libc::c_int);
        }
        str_end = str_end.offset(1);
        str_end;
    }
    (*a).sign = 0 as libc::c_int;
    zset(libzahl_tmp_str_num.as_mut_ptr(), libzahl_const_1e19.as_mut_ptr());
    's_183: {
        let mut current_block_36: u64;
        match str_end.offset_from(str) as libc::c_long
            % 19 as libc::c_int as libc::c_long
        {
            0 => {
                current_block_36 = 3358581507131681108;
            }
            18 => {
                current_block_36 = 3944213605077522399;
            }
            17 => {
                current_block_36 = 12339854756344729914;
            }
            16 => {
                current_block_36 = 10175266677904727412;
            }
            15 => {
                current_block_36 = 6498688920109107990;
            }
            14 => {
                current_block_36 = 11747209804061434085;
            }
            13 => {
                current_block_36 = 16063428799899300179;
            }
            12 => {
                current_block_36 = 11000303109346710806;
            }
            11 => {
                current_block_36 = 11011827531525773283;
            }
            10 => {
                current_block_36 = 11174857987450763079;
            }
            9 => {
                current_block_36 = 15963427805601190657;
            }
            8 => {
                current_block_36 = 8607406065398072226;
            }
            7 => {
                current_block_36 = 3803618718880307134;
            }
            6 => {
                current_block_36 = 5532183643164662141;
            }
            5 => {
                current_block_36 = 11429293225007286200;
            }
            4 => {
                current_block_36 = 13265596162993994991;
            }
            3 => {
                current_block_36 = 17434292904315486127;
            }
            2 => {
                current_block_36 = 2280485322995220430;
            }
            1 => {
                current_block_36 = 12207942584542961383;
            }
            _ => {
                current_block_36 = 10692455896603418738;
            }
        }
        loop {
            match current_block_36 {
                10692455896603418738 => {
                    break 's_183;
                }
                3358581507131681108 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh0 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh0 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 3944213605077522399;
                }
                3944213605077522399 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh1 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh1 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 12339854756344729914;
                }
                12339854756344729914 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh2 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh2 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 10175266677904727412;
                }
                10175266677904727412 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh3 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh3 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 6498688920109107990;
                }
                6498688920109107990 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh4 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh4 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 11747209804061434085;
                }
                11747209804061434085 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh5 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh5 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 16063428799899300179;
                }
                16063428799899300179 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh6 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh6 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 11000303109346710806;
                }
                11000303109346710806 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh7 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh7 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 11011827531525773283;
                }
                11011827531525773283 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh8 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh8 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 11174857987450763079;
                }
                11174857987450763079 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh9 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh9 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 15963427805601190657;
                }
                15963427805601190657 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh10 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh10 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 8607406065398072226;
                }
                8607406065398072226 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh11 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh11 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 3803618718880307134;
                }
                3803618718880307134 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh12 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh12 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 5532183643164662141;
                }
                5532183643164662141 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh13 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh13 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 11429293225007286200;
                }
                11429293225007286200 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh14 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh14 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 13265596162993994991;
                }
                13265596162993994991 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh15 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh15 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 17434292904315486127;
                }
                17434292904315486127 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh16 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh16 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 2280485322995220430;
                }
                2280485322995220430 => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh17 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh17 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    current_block_36 = 12207942584542961383;
                }
                _ => {
                    temp = temp.wrapping_mul(10 as libc::c_int as libc::c_ulonglong);
                    let fresh18 = str;
                    str = str.offset(1);
                    temp = temp
                        .wrapping_add(
                            (*fresh18 as libc::c_int & 15 as libc::c_int)
                                as libc::c_ulonglong,
                        );
                    if !(temp == 0) {
                        *((*libzahl_tmp_str_num.as_mut_ptr()).chars)
                            .offset(0 as libc::c_int as isize) = temp as zahl_char_t;
                        zadd(a, a, libzahl_tmp_str_num.as_mut_ptr());
                    }
                    if !(*str != 0) {
                        current_block_36 = 10692455896603418738;
                        continue;
                    }
                    zmul(a, a, libzahl_const_1e19.as_mut_ptr());
                    temp = 0 as libc::c_int as libc::c_ulonglong;
                    current_block_36 = 3358581507131681108;
                }
            }
        }
    }
    if (neg != 0) as libc::c_int as libc::c_long != 0 {
        (*a).sign = -zsignum(a);
    }
    return 0 as libc::c_int;
}
