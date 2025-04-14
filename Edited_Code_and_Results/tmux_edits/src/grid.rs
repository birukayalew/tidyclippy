use ::libc;
use std::ptr::{write_unaligned, null_mut};
extern "C" {
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strlcat(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
        __n: size_t,
    ) -> size_t;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t) -> *mut libc::c_void;
    fn xsnprintf(
        _: *mut libc::c_char,
        _: size_t,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn colour_split_rgb(_: libc::c_int, _: *mut u_char, _: *mut u_char, _: *mut u_char);
    fn utf8_set(_: *mut utf8_data, _: u_char);
    fn log_debug(_: *const libc::c_char, _: ...);
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
}
pub type __u_char = libc::c_uchar;
pub type __u_short = libc::c_ushort;
pub type __u_int = libc::c_uint;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct grid_cell {
    pub flags: u_char,
    pub attr: u_short,
    pub fg: libc::c_int,
    pub bg: libc::c_int,
    pub data: utf8_data,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct utf8_data {
    pub data: [u_char; 9],
    pub have: u_char,
    pub size: u_char,
    pub width: u_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct grid {
    pub flags: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub hscrolled: u_int,
    pub hsize: u_int,
    pub hlimit: u_int,
    pub linedata: *mut grid_line,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct grid_line {
    pub cellused: u_int,
    pub cellsize: u_int,
    pub celldata: *mut grid_cell_entry,
    pub extdsize: u_int,
    pub extddata: *mut grid_cell,
    pub flags: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C, packed)]
pub struct grid_cell_entry {
    pub flags: u_char,
    pub c2rust_unnamed: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub offset: u_int,
    pub data: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub mask: u_int,
    pub code: u_int,
}
#[no_mangle]
pub static mut grid_default_cell: grid_cell = {
    let mut init = grid_cell {
        flags: 0 as libc::c_int as u_char,
        attr: 0 as libc::c_int as u_short,
        fg: 8 as libc::c_int,
        bg: 8 as libc::c_int,
        data: {
            let mut init = utf8_data {
                data: [' ' as i32 as u_char, 0, 0, 0, 0, 0, 0, 0, 0],
                have: 0 as libc::c_int as u_char,
                size: 1 as libc::c_int as u_char,
                width: 1 as libc::c_int as u_char,
            };
            init
        },
    };
    init
};
static mut grid_default_entry: grid_cell_entry = {
    let mut init = grid_cell_entry {
        flags: 0 as libc::c_int as u_char,
        c2rust_unnamed: C2RustUnnamed {
            data: {
                let mut init = C2RustUnnamed_0 {
                    attr: 0 as libc::c_int as u_char,
                    fg: 8 as libc::c_int as u_char,
                    bg: 8 as libc::c_int as u_char,
                    data: ' ' as i32 as u_char,
                };
                init
            },
        },
    };
    init
};
unsafe extern "C" fn grid_store_cell(
    mut gce: *mut grid_cell_entry,
    mut gc: *const grid_cell,
    mut c: u_char,
) {
    (*gce).flags = (*gc).flags;
    (*gce).c2rust_unnamed.data.fg = ((*gc).fg & 0xff as libc::c_int) as u_char;
    if (*gc).fg & 0x1000000 as libc::c_int != 0 {
        (*gce).flags = ((*gce).flags as libc::c_int | 0x1 as libc::c_int) as u_char;
    }
    (*gce).c2rust_unnamed.data.bg = ((*gc).bg & 0xff as libc::c_int) as u_char;
    if (*gc).bg & 0x1000000 as libc::c_int != 0 {
        (*gce).flags = ((*gce).flags as libc::c_int | 0x2 as libc::c_int) as u_char;
    }
    (*gce).c2rust_unnamed.data.attr = (*gc).attr as u_char;
    (*gce).c2rust_unnamed.data.data = c;
}
unsafe extern "C" fn grid_need_extended_cell(
    mut gce: *const grid_cell_entry,
    mut gc: *const grid_cell,
) -> libc::c_int {
    if (*gce).flags as libc::c_int & 0x8 as libc::c_int != 0 {
        return 1 as libc::c_int;
    }
    if (*gc).attr as libc::c_int > 0xff as libc::c_int {
        return 1 as libc::c_int;
    }
    if (*gc).data.size as libc::c_int != 1 as libc::c_int
        || (*gc).data.width as libc::c_int != 1 as libc::c_int
    {
        return 1 as libc::c_int;
    }
    if (*gc).fg & 0x2000000 as libc::c_int != 0
        || (*gc).bg & 0x2000000 as libc::c_int != 0
    {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
unsafe extern "C" fn grid_compact_line(mut gl: *mut grid_line) {
    let mut new_extdsize: libc::c_int = 0 as libc::c_int;
    let mut new_extddata: *mut grid_cell = 0 as *mut grid_cell;
    let mut gce: *mut grid_cell_entry = 0 as *mut grid_cell_entry;
    let mut gc: *mut grid_cell = 0 as *mut grid_cell;
    let mut px: u_int = 0;
    let mut idx: u_int = 0;
    if (*gl).extdsize == 0 as libc::c_int as u_int {
        return;
    }
    px = 0 as libc::c_int as u_int;
    while px < (*gl).cellsize {
        gce = &mut *((*gl).celldata).offset(px as isize) as *mut grid_cell_entry;
        if (*gce).flags as libc::c_int & 0x8 as libc::c_int != 0 {
            new_extdsize += 1;
            new_extdsize;
        }
        px = px.wrapping_add(1);
        px;
    }
    if new_extdsize == 0 as libc::c_int {
        free((*gl).extddata as *mut libc::c_void);
        (*gl).extddata = 0 as *mut grid_cell;
        (*gl).extdsize = 0 as libc::c_int as u_int;
        return;
    }
    new_extddata = xreallocarray(
        0 as *mut libc::c_void,
        new_extdsize as size_t,
        ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
    ) as *mut grid_cell;
    idx = 0 as libc::c_int as u_int;
    px = 0 as libc::c_int as u_int;
    while px < (*gl).cellsize {
        gce = &mut *((*gl).celldata).offset(px as isize) as *mut grid_cell_entry;
        if (*gce).flags as libc::c_int & 0x8 as libc::c_int != 0 {
            gc = &mut *((*gl).extddata).offset((*gce).c2rust_unnamed.offset as isize)
                as *mut grid_cell;
            memcpy(
                &mut *new_extddata.offset(idx as isize) as *mut grid_cell
                    as *mut libc::c_void,
                gc as *const libc::c_void,
                ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
            );
            let fresh0 = idx;
            idx = idx.wrapping_add(1);
            (*gce).c2rust_unnamed.offset = fresh0;
        }
        px = px.wrapping_add(1);
        px;
    }
    free((*gl).extddata as *mut libc::c_void);
    (*gl).extddata = new_extddata;
    (*gl).extdsize = new_extdsize as u_int;
}
unsafe extern "C" fn grid_extended_cell(
    mut gl: *mut grid_line,
    mut gce: *mut grid_cell_entry,
    mut gc: *const grid_cell,
) -> *mut grid_cell {
    let mut gcp: *mut grid_cell = 0 as *mut grid_cell;
    (*gl).flags |= 0x2 as libc::c_int;
    if !((*gce).flags as libc::c_int) & 0x8 as libc::c_int != 0 {
        (*gl)
            .extddata = xreallocarray(
            (*gl).extddata as *mut libc::c_void,
            ((*gl).extdsize).wrapping_add(1 as libc::c_int as u_int) as size_t,
            ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
        ) as *mut grid_cell;
        let fresh1 = (*gl).extdsize;
        (*gl).extdsize = ((*gl).extdsize).wrapping_add(1);
        (*gce).c2rust_unnamed.offset = fresh1;
        (*gce).flags = ((*gc).flags as libc::c_int | 0x8 as libc::c_int) as u_char;
    }
    if (*gce).c2rust_unnamed.offset >= (*gl).extdsize {
        fatalx(b"offset too big\0" as *const u8 as *const libc::c_char);
    }
    gcp = &mut *((*gl).extddata).offset((*gce).c2rust_unnamed.offset as isize)
        as *mut grid_cell;
    memcpy(
        gcp as *mut libc::c_void,
        gc as *const libc::c_void,
        ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    return gcp;
}
unsafe extern "C" fn grid_clear_cell(
    mut gd: *mut grid,
    mut px: u_int,
    mut py: u_int,
    mut bg: u_int,
) {
    let mut gl: *mut grid_line = &mut *((*gd).linedata).offset(py as isize)
        as *mut grid_line;
    let mut gce: *mut grid_cell_entry = &mut *((*gl).celldata).offset(px as isize)
        as *mut grid_cell_entry;
    let mut gc: *mut grid_cell = 0 as *mut grid_cell;
    memcpy(
        gce as *mut libc::c_void,
        &grid_default_entry as *const grid_cell_entry as *const libc::c_void,
        ::core::mem::size_of::<grid_cell_entry>() as libc::c_ulong,
    );
    if bg & 0x2000000 as libc::c_int as u_int != 0 {
        gc = grid_extended_cell(gl, gce, &grid_default_cell);
        (*gc).bg = bg as libc::c_int;
    } else {
        if bg & 0x1000000 as libc::c_int as u_int != 0 {
            (*gce).flags = ((*gce).flags as libc::c_int | 0x2 as libc::c_int) as u_char;
        }
        (*gce).c2rust_unnamed.data.bg = bg as u_char;
    };
}
unsafe extern "C" fn grid_check_y(
    mut gd: *mut grid,
    mut from: *const libc::c_char,
    mut py: u_int,
) -> libc::c_int {
    if py >= ((*gd).hsize).wrapping_add((*gd).sy) {
        log_debug(
            b"%s: y out of range: %u\0" as *const u8 as *const libc::c_char,
            from,
            py,
        );
        return -(1 as libc::c_int);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn grid_cells_equal(
    mut gca: *const grid_cell,
    mut gcb: *const grid_cell,
) -> libc::c_int {
    if (*gca).fg != (*gcb).fg || (*gca).bg != (*gcb).bg {
        return 0 as libc::c_int;
    }
    if (*gca).attr as libc::c_int != (*gcb).attr as libc::c_int
        || (*gca).flags as libc::c_int != (*gcb).flags as libc::c_int
    {
        return 0 as libc::c_int;
    }
    if (*gca).data.width as libc::c_int != (*gcb).data.width as libc::c_int {
        return 0 as libc::c_int;
    }
    if (*gca).data.size as libc::c_int != (*gcb).data.size as libc::c_int {
        return 0 as libc::c_int;
    }
    return (memcmp(
        ((*gca).data.data).as_ptr() as *const libc::c_void,
        ((*gcb).data.data).as_ptr() as *const libc::c_void,
        (*gca).data.size as libc::c_ulong,
    ) == 0 as libc::c_int) as libc::c_int;
}
unsafe extern "C" fn grid_free_line(mut gd: *mut grid, mut py: u_int) {
    let line = (*gd).linedata.offset(py as isize);

    free((*line).celldata as *mut libc::c_void);
    let celldata_ptr = std::ptr::addr_of_mut!((*line).celldata) as *mut *mut grid_cell_entry;
    write_unaligned(celldata_ptr, null_mut());

    free((*line).extddata as *mut libc::c_void);
    let extddata_ptr = std::ptr::addr_of_mut!((*line).extddata) as *mut *mut grid_cell;
    write_unaligned(extddata_ptr, null_mut());
}
unsafe extern "C" fn grid_free_lines(mut gd: *mut grid, mut py: u_int, mut ny: u_int) {
    let mut yy: u_int = 0;
    yy = py;
    while yy < py.wrapping_add(ny) {
        grid_free_line(gd, yy);
        yy = yy.wrapping_add(1);
        yy;
    }
}
#[no_mangle]
pub unsafe extern "C" fn grid_create(
    mut sx: u_int,
    mut sy: u_int,
    mut hlimit: u_int,
) -> *mut grid {
    let mut gd: *mut grid = 0 as *mut grid;
    gd = xmalloc(::core::mem::size_of::<grid>() as libc::c_ulong) as *mut grid;
    (*gd).sx = sx;
    (*gd).sy = sy;
    (*gd).flags = 0x1 as libc::c_int;
    (*gd).hscrolled = 0 as libc::c_int as u_int;
    (*gd).hsize = 0 as libc::c_int as u_int;
    (*gd).hlimit = hlimit;
    if (*gd).sy != 0 as libc::c_int as u_int {
        (*gd)
            .linedata = xcalloc(
            (*gd).sy as size_t,
            ::core::mem::size_of::<grid_line>() as libc::c_ulong,
        ) as *mut grid_line;
    } else {
        (*gd).linedata = 0 as *mut grid_line;
    }
    return gd;
}
#[no_mangle]
pub unsafe extern "C" fn grid_destroy(mut gd: *mut grid) {
    grid_free_lines(gd, 0 as libc::c_int as u_int, ((*gd).hsize).wrapping_add((*gd).sy));
    free((*gd).linedata as *mut libc::c_void);
    free(gd as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn grid_compare(
    mut ga: *mut grid,
    mut gb: *mut grid,
) -> libc::c_int {
    let mut gla: *mut grid_line = 0 as *mut grid_line;
    let mut glb: *mut grid_line = 0 as *mut grid_line;
    let mut gca: grid_cell = grid_cell {
        flags: 0,
        attr: 0,
        fg: 0,
        bg: 0,
        data: utf8_data {
            data: [0; 9],
            have: 0,
            size: 0,
            width: 0,
        },
    };
    let mut gcb: grid_cell = grid_cell {
        flags: 0,
        attr: 0,
        fg: 0,
        bg: 0,
        data: utf8_data {
            data: [0; 9],
            have: 0,
            size: 0,
            width: 0,
        },
    };
    let mut xx: u_int = 0;
    let mut yy: u_int = 0;
    if (*ga).sx != (*gb).sx || (*ga).sy != (*gb).sy {
        return 1 as libc::c_int;
    }
    yy = 0 as libc::c_int as u_int;
    while yy < (*ga).sy {
        gla = &mut *((*ga).linedata).offset(yy as isize) as *mut grid_line;
        glb = &mut *((*gb).linedata).offset(yy as isize) as *mut grid_line;
        if (*gla).cellsize != (*glb).cellsize {
            return 1 as libc::c_int;
        }
        xx = 0 as libc::c_int as u_int;
        while xx < (*gla).cellsize {
            grid_get_cell(ga, xx, yy, &mut gca);
            grid_get_cell(gb, xx, yy, &mut gcb);
            if grid_cells_equal(&mut gca, &mut gcb) == 0 {
                return 1 as libc::c_int;
            }
            xx = xx.wrapping_add(1);
            xx;
        }
        yy = yy.wrapping_add(1);
        yy;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn grid_collect_history(mut gd: *mut grid) {
    let mut ny: u_int = 0;
    if (*gd).hsize == 0 as libc::c_int as u_int || (*gd).hsize < (*gd).hlimit {
        return;
    }
    ny = (*gd).hlimit / 10 as libc::c_int as u_int;
    if ny < 1 as libc::c_int as u_int {
        ny = 1 as libc::c_int as u_int;
    }
    if ny > (*gd).hsize {
        ny = (*gd).hsize;
    }
    grid_free_lines(gd, 0 as libc::c_int as u_int, ny);
    memmove(
        &mut *((*gd).linedata).offset(0 as libc::c_int as isize) as *mut grid_line
            as *mut libc::c_void,
        &mut *((*gd).linedata).offset(ny as isize) as *mut grid_line
            as *const libc::c_void,
        (((*gd).hsize).wrapping_add((*gd).sy).wrapping_sub(ny) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<grid_line>() as libc::c_ulong),
    );
    (*gd).hsize = ((*gd).hsize).wrapping_sub(ny);
    if (*gd).hscrolled > (*gd).hsize {
        (*gd).hscrolled = (*gd).hsize;
    }
}
#[no_mangle]
pub unsafe extern "C" fn grid_scroll_history(mut gd: *mut grid, mut bg: u_int) {
    let mut yy: u_int = 0;
    yy = ((*gd).hsize).wrapping_add((*gd).sy);
    (*gd)
        .linedata = xreallocarray(
        (*gd).linedata as *mut libc::c_void,
        yy.wrapping_add(1 as libc::c_int as u_int) as size_t,
        ::core::mem::size_of::<grid_line>() as libc::c_ulong,
    ) as *mut grid_line;
    grid_empty_line(gd, yy, bg);
    (*gd).hscrolled = ((*gd).hscrolled).wrapping_add(1);
    (*gd).hscrolled;
    grid_compact_line(&mut *((*gd).linedata).offset((*gd).hsize as isize));
    (*gd).hsize = ((*gd).hsize).wrapping_add(1);
    (*gd).hsize;
}
#[no_mangle]
pub unsafe extern "C" fn grid_clear_history(mut gd: *mut grid) {
    grid_free_lines(gd, 0 as libc::c_int as u_int, (*gd).hsize);
    memmove(
        &mut *((*gd).linedata).offset(0 as libc::c_int as isize) as *mut grid_line
            as *mut libc::c_void,
        &mut *((*gd).linedata).offset((*gd).hsize as isize) as *mut grid_line
            as *const libc::c_void,
        ((*gd).sy as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<grid_line>() as libc::c_ulong),
    );
    (*gd).hscrolled = 0 as libc::c_int as u_int;
    (*gd).hsize = 0 as libc::c_int as u_int;
    (*gd)
        .linedata = xreallocarray(
        (*gd).linedata as *mut libc::c_void,
        (*gd).sy as size_t,
        ::core::mem::size_of::<grid_line>() as libc::c_ulong,
    ) as *mut grid_line;
}
#[no_mangle]
pub unsafe extern "C" fn grid_scroll_history_region(
    mut gd: *mut grid,
    mut upper: u_int,
    mut lower: u_int,
    mut bg: u_int,
) {
    let mut gl_history: *mut grid_line = 0 as *mut grid_line;
    let mut gl_upper: *mut grid_line = 0 as *mut grid_line;
    let mut yy: u_int = 0;
    yy = ((*gd).hsize).wrapping_add((*gd).sy);
    (*gd)
        .linedata = xreallocarray(
        (*gd).linedata as *mut libc::c_void,
        yy.wrapping_add(1 as libc::c_int as u_int) as size_t,
        ::core::mem::size_of::<grid_line>() as libc::c_ulong,
    ) as *mut grid_line;
    gl_history = &mut *((*gd).linedata).offset((*gd).hsize as isize) as *mut grid_line;
    memmove(
        gl_history.offset(1 as libc::c_int as isize) as *mut libc::c_void,
        gl_history as *const libc::c_void,
        ((*gd).sy as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<grid_line>() as libc::c_ulong),
    );
    upper = upper.wrapping_add(1);
    upper;
    gl_upper = &mut *((*gd).linedata).offset(upper as isize) as *mut grid_line;
    lower = lower.wrapping_add(1);
    lower;
    memcpy(
        gl_history as *mut libc::c_void,
        gl_upper as *const libc::c_void,
        ::core::mem::size_of::<grid_line>() as libc::c_ulong,
    );
    memmove(
        gl_upper as *mut libc::c_void,
        gl_upper.offset(1 as libc::c_int as isize) as *const libc::c_void,
        (lower.wrapping_sub(upper) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<grid_line>() as libc::c_ulong),
    );
    grid_empty_line(gd, lower, bg);
    (*gd).hscrolled = ((*gd).hscrolled).wrapping_add(1);
    (*gd).hscrolled;
    (*gd).hsize = ((*gd).hsize).wrapping_add(1);
    (*gd).hsize;
}
unsafe extern "C" fn grid_expand_line(
    mut gd: *mut grid,
    mut py: u_int,
    mut sx: u_int,
    mut bg: u_int,
) {
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut xx: u_int = 0;
    gl = &mut *((*gd).linedata).offset(py as isize) as *mut grid_line;
    if sx <= (*gl).cellsize {
        return;
    }
    if sx < (*gd).sx / 4 as libc::c_int as u_int {
        sx = (*gd).sx / 4 as libc::c_int as u_int;
    } else if sx < (*gd).sx / 2 as libc::c_int as u_int {
        sx = (*gd).sx / 2 as libc::c_int as u_int;
    } else {
        sx = (*gd).sx;
    }
    (*gl)
        .celldata = xreallocarray(
        (*gl).celldata as *mut libc::c_void,
        sx as size_t,
        ::core::mem::size_of::<grid_cell_entry>() as libc::c_ulong,
    ) as *mut grid_cell_entry;
    xx = (*gl).cellsize;
    while xx < sx {
        grid_clear_cell(gd, xx, py, bg);
        xx = xx.wrapping_add(1);
        xx;
    }
    (*gl).cellsize = sx;
}
unsafe extern "C" fn grid_empty_line(mut gd: *mut grid, mut py: u_int, mut bg: u_int) {
    memset(
        &mut *((*gd).linedata).offset(py as isize) as *mut grid_line
            as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<grid_line>() as libc::c_ulong,
    );
    if bg != 8 as libc::c_int as u_int {
        grid_expand_line(gd, py, (*gd).sx, bg);
    }
}
#[no_mangle]
pub unsafe extern "C" fn grid_peek_line(
    mut gd: *mut grid,
    mut py: u_int,
) -> *const grid_line {
    if grid_check_y(
        gd,
        (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"grid_peek_line\0"))
            .as_ptr(),
        py,
    ) != 0 as libc::c_int
    {
        return 0 as *const grid_line;
    }
    return &mut *((*gd).linedata).offset(py as isize) as *mut grid_line;
}
unsafe extern "C" fn grid_get_cell1(
    mut gl: *mut grid_line,
    mut px: u_int,
    mut gc: *mut grid_cell,
) {
    let mut gce: *mut grid_cell_entry = &mut *((*gl).celldata).offset(px as isize)
        as *mut grid_cell_entry;
    if (*gce).flags as libc::c_int & 0x8 as libc::c_int != 0 {
        if (*gce).c2rust_unnamed.offset >= (*gl).extdsize {
            memcpy(
                gc as *mut libc::c_void,
                &grid_default_cell as *const grid_cell as *const libc::c_void,
                ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
            );
        } else {
            memcpy(
                gc as *mut libc::c_void,
                &mut *((*gl).extddata).offset((*gce).c2rust_unnamed.offset as isize)
                    as *mut grid_cell as *const libc::c_void,
                ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
            );
        }
        return;
    }
    (*gc)
        .flags = ((*gce).flags as libc::c_int
        & !(0x1 as libc::c_int | 0x2 as libc::c_int)) as u_char;
    (*gc).attr = (*gce).c2rust_unnamed.data.attr as u_short;
    (*gc).fg = (*gce).c2rust_unnamed.data.fg as libc::c_int;
    if (*gce).flags as libc::c_int & 0x1 as libc::c_int != 0 {
        (*gc).fg |= 0x1000000 as libc::c_int;
    }
    (*gc).bg = (*gce).c2rust_unnamed.data.bg as libc::c_int;
    if (*gce).flags as libc::c_int & 0x2 as libc::c_int != 0 {
        (*gc).bg |= 0x1000000 as libc::c_int;
    }
    utf8_set(&mut (*gc).data, (*gce).c2rust_unnamed.data.data);
}
#[no_mangle]
pub unsafe extern "C" fn grid_get_cell(
    mut gd: *mut grid,
    mut px: u_int,
    mut py: u_int,
    mut gc: *mut grid_cell,
) {
    if grid_check_y(
        gd,
        (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"grid_get_cell\0"))
            .as_ptr(),
        py,
    ) != 0 as libc::c_int || px >= (*((*gd).linedata).offset(py as isize)).cellsize
    {
        memcpy(
            gc as *mut libc::c_void,
            &grid_default_cell as *const grid_cell as *const libc::c_void,
            ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
        );
        return;
    }
    return grid_get_cell1(&mut *((*gd).linedata).offset(py as isize), px, gc);
}
#[no_mangle]
pub unsafe extern "C" fn grid_set_cell(
    mut gd: *mut grid,
    mut px: u_int,
    mut py: u_int,
    mut gc: *const grid_cell,
) {
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut gce: *mut grid_cell_entry = 0 as *mut grid_cell_entry;
    if grid_check_y(
        gd,
        (*::core::mem::transmute::<&[u8; 14], &[libc::c_char; 14]>(b"grid_set_cell\0"))
            .as_ptr(),
        py,
    ) != 0 as libc::c_int
    {
        return;
    }
    grid_expand_line(
        gd,
        py,
        px.wrapping_add(1 as libc::c_int as u_int),
        8 as libc::c_int as u_int,
    );
    gl = &mut *((*gd).linedata).offset(py as isize) as *mut grid_line;
    if px.wrapping_add(1 as libc::c_int as u_int) > (*gl).cellused {
        (*gl).cellused = px.wrapping_add(1 as libc::c_int as u_int);
    }
    gce = &mut *((*gl).celldata).offset(px as isize) as *mut grid_cell_entry;
    if grid_need_extended_cell(gce, gc) != 0 {
        grid_extended_cell(gl, gce, gc);
    } else {
        grid_store_cell(gce, gc, (*gc).data.data[0 as libc::c_int as usize]);
    };
}
#[no_mangle]
pub unsafe extern "C" fn grid_set_cells(
    mut gd: *mut grid,
    mut px: u_int,
    mut py: u_int,
    mut gc: *const grid_cell,
    mut s: *const libc::c_char,
    mut slen: size_t,
) {
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut gce: *mut grid_cell_entry = 0 as *mut grid_cell_entry;
    let mut gcp: *mut grid_cell = 0 as *mut grid_cell;
    let mut i: u_int = 0;
    if grid_check_y(
        gd,
        (*::core::mem::transmute::<&[u8; 15], &[libc::c_char; 15]>(b"grid_set_cells\0"))
            .as_ptr(),
        py,
    ) != 0 as libc::c_int
    {
        return;
    }
    grid_expand_line(
        gd,
        py,
        (px as size_t).wrapping_add(slen) as u_int,
        8 as libc::c_int as u_int,
    );
    gl = &mut *((*gd).linedata).offset(py as isize) as *mut grid_line;
    if (px as size_t).wrapping_add(slen) > (*gl).cellused as size_t {
        (*gl).cellused = (px as size_t).wrapping_add(slen) as u_int;
    }
    i = 0 as libc::c_int as u_int;
    while (i as size_t) < slen {
        gce = &mut *((*gl).celldata).offset(px.wrapping_add(i) as isize)
            as *mut grid_cell_entry;
        if grid_need_extended_cell(gce, gc) != 0 {
            gcp = grid_extended_cell(gl, gce, gc);
            utf8_set(&mut (*gcp).data, *s.offset(i as isize) as u_char);
        } else {
            grid_store_cell(gce, gc, *s.offset(i as isize) as u_char);
        }
        i = i.wrapping_add(1);
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn grid_clear(
    mut gd: *mut grid,
    mut px: u_int,
    mut py: u_int,
    mut nx: u_int,
    mut ny: u_int,
    mut bg: u_int,
) {
    let mut xx: u_int = 0;
    let mut yy: u_int = 0;
    if nx == 0 as libc::c_int as u_int || ny == 0 as libc::c_int as u_int {
        return;
    }
    if px == 0 as libc::c_int as u_int && nx == (*gd).sx {
        grid_clear_lines(gd, py, ny, bg);
        return;
    }
    if grid_check_y(
        gd,
        (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"grid_clear\0"))
            .as_ptr(),
        py,
    ) != 0 as libc::c_int
    {
        return;
    }
    if grid_check_y(
        gd,
        (*::core::mem::transmute::<&[u8; 11], &[libc::c_char; 11]>(b"grid_clear\0"))
            .as_ptr(),
        py.wrapping_add(ny).wrapping_sub(1 as libc::c_int as u_int),
    ) != 0 as libc::c_int
    {
        return;
    }
    yy = py;
    while yy < py.wrapping_add(ny) {
        if px.wrapping_add(nx) >= (*gd).sx
            && px < (*((*gd).linedata).offset(yy as isize)).cellused
        {
            (*((*gd).linedata).offset(yy as isize)).cellused = px;
        }
        if !(px > (*((*gd).linedata).offset(yy as isize)).cellsize
            && bg == 8 as libc::c_int as u_int)
        {
            if px.wrapping_add(nx) >= (*((*gd).linedata).offset(yy as isize)).cellsize
                && bg == 8 as libc::c_int as u_int
            {
                (*((*gd).linedata).offset(yy as isize)).cellsize = px;
            } else {
                grid_expand_line(gd, yy, px.wrapping_add(nx), 8 as libc::c_int as u_int);
                xx = px;
                while xx < px.wrapping_add(nx) {
                    grid_clear_cell(gd, xx, yy, bg);
                    xx = xx.wrapping_add(1);
                    xx;
                }
            }
        }
        yy = yy.wrapping_add(1);
        yy;
    }
}
#[no_mangle]
pub unsafe extern "C" fn grid_clear_lines(
    mut gd: *mut grid,
    mut py: u_int,
    mut ny: u_int,
    mut bg: u_int,
) {
    let mut yy: u_int = 0;
    if ny == 0 as libc::c_int as u_int {
        return;
    }
    if grid_check_y(
        gd,
        (*::core::mem::transmute::<
            &[u8; 17],
            &[libc::c_char; 17],
        >(b"grid_clear_lines\0"))
            .as_ptr(),
        py,
    ) != 0 as libc::c_int
    {
        return;
    }
    if grid_check_y(
        gd,
        (*::core::mem::transmute::<
            &[u8; 17],
            &[libc::c_char; 17],
        >(b"grid_clear_lines\0"))
            .as_ptr(),
        py.wrapping_add(ny).wrapping_sub(1 as libc::c_int as u_int),
    ) != 0 as libc::c_int
    {
        return;
    }
    yy = py;
    while yy < py.wrapping_add(ny) {
        grid_free_line(gd, yy);
        grid_empty_line(gd, yy, bg);
        yy = yy.wrapping_add(1);
        yy;
    }
}
#[no_mangle]
pub unsafe extern "C" fn grid_move_lines(
    mut gd: *mut grid,
    mut dy: u_int,
    mut py: u_int,
    mut ny: u_int,
    mut bg: u_int,
) {
    let mut yy: u_int = 0;
    if ny == 0 as libc::c_int as u_int || py == dy {
        return;
    }
    if grid_check_y(
        gd,
        (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"grid_move_lines\0"))
            .as_ptr(),
        py,
    ) != 0 as libc::c_int
    {
        return;
    }
    if grid_check_y(
        gd,
        (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"grid_move_lines\0"))
            .as_ptr(),
        py.wrapping_add(ny).wrapping_sub(1 as libc::c_int as u_int),
    ) != 0 as libc::c_int
    {
        return;
    }
    if grid_check_y(
        gd,
        (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"grid_move_lines\0"))
            .as_ptr(),
        dy,
    ) != 0 as libc::c_int
    {
        return;
    }
    if grid_check_y(
        gd,
        (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"grid_move_lines\0"))
            .as_ptr(),
        dy.wrapping_add(ny).wrapping_sub(1 as libc::c_int as u_int),
    ) != 0 as libc::c_int
    {
        return;
    }
    yy = dy;
    while yy < dy.wrapping_add(ny) {
        if !(yy >= py && yy < py.wrapping_add(ny)) {
            grid_free_line(gd, yy);
        }
        yy = yy.wrapping_add(1);
        yy;
    }
    memmove(
        &mut *((*gd).linedata).offset(dy as isize) as *mut grid_line
            as *mut libc::c_void,
        &mut *((*gd).linedata).offset(py as isize) as *mut grid_line
            as *const libc::c_void,
        (ny as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<grid_line>() as libc::c_ulong),
    );
    yy = py;
    while yy < py.wrapping_add(ny) {
        if yy < dy || yy >= dy.wrapping_add(ny) {
            grid_empty_line(gd, yy, bg);
        }
        yy = yy.wrapping_add(1);
        yy;
    }
}
#[no_mangle]
pub unsafe extern "C" fn grid_move_cells(
    mut gd: *mut grid,
    mut dx: u_int,
    mut px: u_int,
    mut py: u_int,
    mut nx: u_int,
    mut bg: u_int,
) {
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut xx: u_int = 0;
    if nx == 0 as libc::c_int as u_int || px == dx {
        return;
    }
    if grid_check_y(
        gd,
        (*::core::mem::transmute::<&[u8; 16], &[libc::c_char; 16]>(b"grid_move_cells\0"))
            .as_ptr(),
        py,
    ) != 0 as libc::c_int
    {
        return;
    }
    gl = &mut *((*gd).linedata).offset(py as isize) as *mut grid_line;
    grid_expand_line(gd, py, px.wrapping_add(nx), 8 as libc::c_int as u_int);
    grid_expand_line(gd, py, dx.wrapping_add(nx), 8 as libc::c_int as u_int);
    memmove(
        &mut *((*gl).celldata).offset(dx as isize) as *mut grid_cell_entry
            as *mut libc::c_void,
        &mut *((*gl).celldata).offset(px as isize) as *mut grid_cell_entry
            as *const libc::c_void,
        (nx as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<grid_cell_entry>() as libc::c_ulong),
    );
    if dx.wrapping_add(nx) > (*gl).cellused {
        (*gl).cellused = dx.wrapping_add(nx);
    }
    xx = px;
    while xx < px.wrapping_add(nx) {
        if !(xx >= dx && xx < dx.wrapping_add(nx)) {
            grid_clear_cell(gd, xx, py, bg);
        }
        xx = xx.wrapping_add(1);
        xx;
    }
}
unsafe extern "C" fn grid_string_cells_fg(
    mut gc: *const grid_cell,
    mut values: *mut libc::c_int,
) -> size_t {
    let mut n: size_t = 0;
    let mut r: u_char = 0;
    let mut g: u_char = 0;
    let mut b: u_char = 0;
    n = 0 as libc::c_int as size_t;
    if (*gc).fg & 0x1000000 as libc::c_int != 0 {
        let fresh4 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh4 as isize) = 38 as libc::c_int;
        let fresh5 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh5 as isize) = 5 as libc::c_int;
        let fresh6 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh6 as isize) = (*gc).fg & 0xff as libc::c_int;
    } else if (*gc).fg & 0x2000000 as libc::c_int != 0 {
        let fresh7 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh7 as isize) = 38 as libc::c_int;
        let fresh8 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh8 as isize) = 2 as libc::c_int;
        colour_split_rgb((*gc).fg, &mut r, &mut g, &mut b);
        let fresh9 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh9 as isize) = r as libc::c_int;
        let fresh10 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh10 as isize) = g as libc::c_int;
        let fresh11 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh11 as isize) = b as libc::c_int;
    } else {
        match (*gc).fg {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 => {
                let fresh12 = n;
                n = n.wrapping_add(1);
                *values.offset(fresh12 as isize) = (*gc).fg + 30 as libc::c_int;
            }
            8 => {
                let fresh13 = n;
                n = n.wrapping_add(1);
                *values.offset(fresh13 as isize) = 39 as libc::c_int;
            }
            90 | 91 | 92 | 93 | 94 | 95 | 96 | 97 => {
                let fresh14 = n;
                n = n.wrapping_add(1);
                *values.offset(fresh14 as isize) = (*gc).fg;
            }
            _ => {}
        }
    }
    return n;
}
unsafe extern "C" fn grid_string_cells_bg(
    mut gc: *const grid_cell,
    mut values: *mut libc::c_int,
) -> size_t {
    let mut n: size_t = 0;
    let mut r: u_char = 0;
    let mut g: u_char = 0;
    let mut b: u_char = 0;
    n = 0 as libc::c_int as size_t;
    if (*gc).bg & 0x1000000 as libc::c_int != 0 {
        let fresh15 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh15 as isize) = 48 as libc::c_int;
        let fresh16 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh16 as isize) = 5 as libc::c_int;
        let fresh17 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh17 as isize) = (*gc).bg & 0xff as libc::c_int;
    } else if (*gc).bg & 0x2000000 as libc::c_int != 0 {
        let fresh18 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh18 as isize) = 48 as libc::c_int;
        let fresh19 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh19 as isize) = 2 as libc::c_int;
        colour_split_rgb((*gc).bg, &mut r, &mut g, &mut b);
        let fresh20 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh20 as isize) = r as libc::c_int;
        let fresh21 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh21 as isize) = g as libc::c_int;
        let fresh22 = n;
        n = n.wrapping_add(1);
        *values.offset(fresh22 as isize) = b as libc::c_int;
    } else {
        match (*gc).bg {
            0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 => {
                let fresh23 = n;
                n = n.wrapping_add(1);
                *values.offset(fresh23 as isize) = (*gc).bg + 40 as libc::c_int;
            }
            8 => {
                let fresh24 = n;
                n = n.wrapping_add(1);
                *values.offset(fresh24 as isize) = 49 as libc::c_int;
            }
            100 | 101 | 102 | 103 | 104 | 105 | 106 | 107 => {
                let fresh25 = n;
                n = n.wrapping_add(1);
                *values.offset(fresh25 as isize) = (*gc).bg - 10 as libc::c_int;
            }
            _ => {}
        }
    }
    return n;
}
unsafe extern "C" fn grid_string_cells_code(
    mut lastgc: *const grid_cell,
    mut gc: *const grid_cell,
    mut buf: *mut libc::c_char,
    mut len: size_t,
    mut escape_c0: libc::c_int,
) {
    let mut oldc: [libc::c_int; 64] = [0; 64];
    let mut newc: [libc::c_int; 64] = [0; 64];
    let mut s: [libc::c_int; 128] = [0; 128];
    let mut noldc: size_t = 0;
    let mut nnewc: size_t = 0;
    let mut n: size_t = 0;
    let mut i: size_t = 0;
    let mut attr: u_int = (*gc).attr as u_int;
    let mut lastattr: u_int = (*lastgc).attr as u_int;
    let mut tmp: [libc::c_char; 64] = [0; 64];
    let mut attrs: [C2RustUnnamed_1; 8] = [
        {
            let mut init = C2RustUnnamed_1 {
                mask: 0x1 as libc::c_int as u_int,
                code: 1 as libc::c_int as u_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_1 {
                mask: 0x2 as libc::c_int as u_int,
                code: 2 as libc::c_int as u_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_1 {
                mask: 0x40 as libc::c_int as u_int,
                code: 3 as libc::c_int as u_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_1 {
                mask: 0x4 as libc::c_int as u_int,
                code: 4 as libc::c_int as u_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_1 {
                mask: 0x8 as libc::c_int as u_int,
                code: 5 as libc::c_int as u_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_1 {
                mask: 0x10 as libc::c_int as u_int,
                code: 7 as libc::c_int as u_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_1 {
                mask: 0x20 as libc::c_int as u_int,
                code: 8 as libc::c_int as u_int,
            };
            init
        },
        {
            let mut init = C2RustUnnamed_1 {
                mask: 0x100 as libc::c_int as u_int,
                code: 9 as libc::c_int as u_int,
            };
            init
        },
    ];
    n = 0 as libc::c_int as size_t;
    i = 0 as libc::c_int as size_t;
    while i
        < (::core::mem::size_of::<[C2RustUnnamed_1; 8]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong)
    {
        if attr & attrs[i as usize].mask == 0 && lastattr & attrs[i as usize].mask != 0 {
            let fresh26 = n;
            n = n.wrapping_add(1);
            s[fresh26 as usize] = 0 as libc::c_int;
            lastattr &= 0x80 as libc::c_int as u_int;
            break;
        } else {
            i = i.wrapping_add(1);
            i;
        }
    }
    i = 0 as libc::c_int as size_t;
    while i
        < (::core::mem::size_of::<[C2RustUnnamed_1; 8]>() as libc::c_ulong)
            .wrapping_div(::core::mem::size_of::<C2RustUnnamed_1>() as libc::c_ulong)
    {
        if attr & attrs[i as usize].mask != 0 && lastattr & attrs[i as usize].mask == 0 {
            let fresh27 = n;
            n = n.wrapping_add(1);
            s[fresh27 as usize] = attrs[i as usize].code as libc::c_int;
        }
        i = i.wrapping_add(1);
        i;
    }
    *buf = '\0' as i32 as libc::c_char;
    if n > 0 as libc::c_int as size_t {
        if escape_c0 != 0 {
            strlcat(buf, b"\\033[\0" as *const u8 as *const libc::c_char, len);
        } else {
            strlcat(buf, b"\x1B[\0" as *const u8 as *const libc::c_char, len);
        }
        i = 0 as libc::c_int as size_t;
        while i < n {
            if i.wrapping_add(1 as libc::c_int as size_t) < n {
                xsnprintf(
                    tmp.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                    b"%d;\0" as *const u8 as *const libc::c_char,
                    s[i as usize],
                );
            } else {
                xsnprintf(
                    tmp.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                    b"%d\0" as *const u8 as *const libc::c_char,
                    s[i as usize],
                );
            }
            strlcat(buf, tmp.as_mut_ptr(), len);
            i = i.wrapping_add(1);
            i;
        }
        strlcat(buf, b"m\0" as *const u8 as *const libc::c_char, len);
    }
    nnewc = grid_string_cells_fg(gc, newc.as_mut_ptr());
    noldc = grid_string_cells_fg(lastgc, oldc.as_mut_ptr());
    if nnewc != noldc
        || memcmp(
            newc.as_mut_ptr() as *const libc::c_void,
            oldc.as_mut_ptr() as *const libc::c_void,
            nnewc.wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) != 0 as libc::c_int
        || n != 0 as libc::c_int as size_t
            && s[0 as libc::c_int as usize] == 0 as libc::c_int
    {
        if escape_c0 != 0 {
            strlcat(buf, b"\\033[\0" as *const u8 as *const libc::c_char, len);
        } else {
            strlcat(buf, b"\x1B[\0" as *const u8 as *const libc::c_char, len);
        }
        i = 0 as libc::c_int as size_t;
        while i < nnewc {
            if i.wrapping_add(1 as libc::c_int as size_t) < nnewc {
                xsnprintf(
                    tmp.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                    b"%d;\0" as *const u8 as *const libc::c_char,
                    newc[i as usize],
                );
            } else {
                xsnprintf(
                    tmp.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                    b"%d\0" as *const u8 as *const libc::c_char,
                    newc[i as usize],
                );
            }
            strlcat(buf, tmp.as_mut_ptr(), len);
            i = i.wrapping_add(1);
            i;
        }
        strlcat(buf, b"m\0" as *const u8 as *const libc::c_char, len);
    }
    nnewc = grid_string_cells_bg(gc, newc.as_mut_ptr());
    noldc = grid_string_cells_bg(lastgc, oldc.as_mut_ptr());
    if nnewc != noldc
        || memcmp(
            newc.as_mut_ptr() as *const libc::c_void,
            oldc.as_mut_ptr() as *const libc::c_void,
            nnewc.wrapping_mul(::core::mem::size_of::<libc::c_int>() as libc::c_ulong),
        ) != 0 as libc::c_int
        || n != 0 as libc::c_int as size_t
            && s[0 as libc::c_int as usize] == 0 as libc::c_int
    {
        if escape_c0 != 0 {
            strlcat(buf, b"\\033[\0" as *const u8 as *const libc::c_char, len);
        } else {
            strlcat(buf, b"\x1B[\0" as *const u8 as *const libc::c_char, len);
        }
        i = 0 as libc::c_int as size_t;
        while i < nnewc {
            if i.wrapping_add(1 as libc::c_int as size_t) < nnewc {
                xsnprintf(
                    tmp.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                    b"%d;\0" as *const u8 as *const libc::c_char,
                    newc[i as usize],
                );
            } else {
                xsnprintf(
                    tmp.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 64]>() as libc::c_ulong,
                    b"%d\0" as *const u8 as *const libc::c_char,
                    newc[i as usize],
                );
            }
            strlcat(buf, tmp.as_mut_ptr(), len);
            i = i.wrapping_add(1);
            i;
        }
        strlcat(buf, b"m\0" as *const u8 as *const libc::c_char, len);
    }
    if attr & 0x80 as libc::c_int as u_int != 0
        && lastattr & 0x80 as libc::c_int as u_int == 0
    {
        if escape_c0 != 0 {
            strlcat(buf, b"\\016\0" as *const u8 as *const libc::c_char, len);
        } else {
            strlcat(buf, b"\x0E\0" as *const u8 as *const libc::c_char, len);
        }
    }
    if attr & 0x80 as libc::c_int as u_int == 0
        && lastattr & 0x80 as libc::c_int as u_int != 0
    {
        if escape_c0 != 0 {
            strlcat(buf, b"\\017\0" as *const u8 as *const libc::c_char, len);
        } else {
            strlcat(buf, b"\x0F\0" as *const u8 as *const libc::c_char, len);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn grid_string_cells(
    mut gd: *mut grid,
    mut px: u_int,
    mut py: u_int,
    mut nx: u_int,
    mut lastgc: *mut *mut grid_cell,
    mut with_codes: libc::c_int,
    mut escape_c0: libc::c_int,
    mut trim: libc::c_int,
) -> *mut libc::c_char {
    let mut gc: grid_cell = grid_cell {
        flags: 0,
        attr: 0,
        fg: 0,
        bg: 0,
        data: utf8_data {
            data: [0; 9],
            have: 0,
            size: 0,
            width: 0,
        },
    };
    static mut lastgc1: grid_cell = grid_cell {
        flags: 0,
        attr: 0,
        fg: 0,
        bg: 0,
        data: utf8_data {
            data: [0; 9],
            have: 0,
            size: 0,
            width: 0,
        },
    };
    let mut data: *const libc::c_char = 0 as *const libc::c_char;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut code: [libc::c_char; 128] = [0; 128];
    let mut len: size_t = 0;
    let mut off: size_t = 0;
    let mut size: size_t = 0;
    let mut codelen: size_t = 0;
    let mut xx: u_int = 0;
    let mut gl: *const grid_line = 0 as *const grid_line;
    if !lastgc.is_null() && (*lastgc).is_null() {
        memcpy(
            &mut lastgc1 as *mut grid_cell as *mut libc::c_void,
            &grid_default_cell as *const grid_cell as *const libc::c_void,
            ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
        );
        *lastgc = &mut lastgc1;
    }
    len = 128 as libc::c_int as size_t;
    buf = xmalloc(len) as *mut libc::c_char;
    off = 0 as libc::c_int as size_t;
    gl = grid_peek_line(gd, py);
    xx = px;
    while xx < px.wrapping_add(nx) {
        if gl.is_null() || xx >= (*gl).cellsize {
            break;
        }
        grid_get_cell(gd, xx, py, &mut gc);
        if !(gc.flags as libc::c_int & 0x4 as libc::c_int != 0) {
            if with_codes != 0 {
                grid_string_cells_code(
                    *lastgc,
                    &mut gc,
                    code.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 128]>() as libc::c_ulong,
                    escape_c0,
                );
                codelen = strlen(code.as_mut_ptr());
                memcpy(
                    *lastgc as *mut libc::c_void,
                    &mut gc as *mut grid_cell as *const libc::c_void,
                    ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
                );
            } else {
                codelen = 0 as libc::c_int as size_t;
            }
            data = (gc.data.data).as_mut_ptr() as *const libc::c_char;
            size = gc.data.size as size_t;
            if escape_c0 != 0 && size == 1 as libc::c_int as size_t
                && *data as libc::c_int == '\\' as i32
            {
                data = b"\\\\\0" as *const u8 as *const libc::c_char;
                size = 2 as libc::c_int as size_t;
            }
            while len
                < off
                    .wrapping_add(size)
                    .wrapping_add(codelen)
                    .wrapping_add(1 as libc::c_int as size_t)
            {
                buf = xreallocarray(
                    buf as *mut libc::c_void,
                    2 as libc::c_int as size_t,
                    len,
                ) as *mut libc::c_char;
                len = len * 2 as libc::c_int as size_t;
            }
            if codelen != 0 as libc::c_int as size_t {
                memcpy(
                    buf.offset(off as isize) as *mut libc::c_void,
                    code.as_mut_ptr() as *const libc::c_void,
                    codelen,
                );
                off = off.wrapping_add(codelen);
            }
            memcpy(
                buf.offset(off as isize) as *mut libc::c_void,
                data as *const libc::c_void,
                size,
            );
            off = off.wrapping_add(size);
        }
        xx = xx.wrapping_add(1);
        xx;
    }
    if trim != 0 {
        while off > 0 as libc::c_int as size_t
            && *buf.offset(off.wrapping_sub(1 as libc::c_int as size_t) as isize)
                as libc::c_int == ' ' as i32
        {
            off = off.wrapping_sub(1);
            off;
        }
    }
    *buf.offset(off as isize) = '\0' as i32 as libc::c_char;
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn grid_duplicate_lines(
    mut dst: *mut grid,
    mut dy: u_int,
    mut src: *mut grid,
    mut sy: u_int,
    mut ny: u_int,
) {
    let mut dstl: *mut grid_line = 0 as *mut grid_line;
    let mut srcl: *mut grid_line = 0 as *mut grid_line;
    let mut yy: u_int = 0;
    if dy.wrapping_add(ny) > ((*dst).hsize).wrapping_add((*dst).sy) {
        ny = ((*dst).hsize).wrapping_add((*dst).sy).wrapping_sub(dy);
    }
    if sy.wrapping_add(ny) > ((*src).hsize).wrapping_add((*src).sy) {
        ny = ((*src).hsize).wrapping_add((*src).sy).wrapping_sub(sy);
    }
    grid_free_lines(dst, dy, ny);
    yy = 0 as libc::c_int as u_int;
    while yy < ny {
        srcl = &mut *((*src).linedata).offset(sy as isize) as *mut grid_line;
        dstl = &mut *((*dst).linedata).offset(dy as isize) as *mut grid_line;
        memcpy(
            dstl as *mut libc::c_void,
            srcl as *const libc::c_void,
            ::core::mem::size_of::<grid_line>() as libc::c_ulong,
        );
        if (*srcl).cellsize != 0 as libc::c_int as u_int {
            (*dstl)
                .celldata = xreallocarray(
                0 as *mut libc::c_void,
                (*srcl).cellsize as size_t,
                ::core::mem::size_of::<grid_cell_entry>() as libc::c_ulong,
            ) as *mut grid_cell_entry;
            memcpy(
                (*dstl).celldata as *mut libc::c_void,
                (*srcl).celldata as *const libc::c_void,
                ((*srcl).cellsize as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<grid_cell_entry>() as libc::c_ulong,
                    ),
            );
        } else {
            (*dstl).celldata = 0 as *mut grid_cell_entry;
        }
        if (*srcl).extdsize != 0 as libc::c_int as u_int {
            (*dstl).extdsize = (*srcl).extdsize;
            (*dstl)
                .extddata = xreallocarray(
                0 as *mut libc::c_void,
                (*dstl).extdsize as size_t,
                ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
            ) as *mut grid_cell;
            memcpy(
                (*dstl).extddata as *mut libc::c_void,
                (*srcl).extddata as *const libc::c_void,
                ((*dstl).extdsize as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<grid_cell>() as libc::c_ulong),
            );
        }
        sy = sy.wrapping_add(1);
        sy;
        dy = dy.wrapping_add(1);
        dy;
        yy = yy.wrapping_add(1);
        yy;
    }
}
unsafe extern "C" fn grid_reflow_dead(mut gl: *mut grid_line) {
    memset(
        gl as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<grid_line>() as libc::c_ulong,
    );
    (*gl).flags = 0x4 as libc::c_int;
}
unsafe extern "C" fn grid_reflow_add(mut gd: *mut grid, mut n: u_int) -> *mut grid_line {
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut sy: u_int = ((*gd).sy).wrapping_add(n);
    (*gd)
        .linedata = xreallocarray(
        (*gd).linedata as *mut libc::c_void,
        sy as size_t,
        ::core::mem::size_of::<grid_line>() as libc::c_ulong,
    ) as *mut grid_line;
    gl = &mut *((*gd).linedata).offset((*gd).sy as isize) as *mut grid_line;
    memset(
        gl as *mut libc::c_void,
        0 as libc::c_int,
        (n as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<grid_line>() as libc::c_ulong),
    );
    (*gd).sy = sy;
    return gl;
}
unsafe extern "C" fn grid_reflow_move(
    mut gd: *mut grid,
    mut from: *mut grid_line,
) -> *mut grid_line {
    let mut to: *mut grid_line = 0 as *mut grid_line;
    to = grid_reflow_add(gd, 1 as libc::c_int as u_int);
    memcpy(
        to as *mut libc::c_void,
        from as *const libc::c_void,
        ::core::mem::size_of::<grid_line>() as libc::c_ulong,
    );
    grid_reflow_dead(from);
    return to;
}
unsafe extern "C" fn grid_reflow_join(
    mut target: *mut grid,
    mut gd: *mut grid,
    mut sx: u_int,
    mut yy: u_int,
    mut width: u_int,
    mut cy: *mut u_int,
    mut already: libc::c_int,
) {
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut from: *mut grid_line = 0 as *mut grid_line;
    let mut gc: grid_cell = grid_cell {
        flags: 0,
        attr: 0,
        fg: 0,
        bg: 0,
        data: utf8_data {
            data: [0; 9],
            have: 0,
            size: 0,
            width: 0,
        },
    };
    let mut lines: u_int = 0;
    let mut want: u_int = 0;
    let mut left: u_int = 0;
    let mut i: u_int = 0;
    let mut to: u_int = 0;
    let mut line: u_int = 0;
    let mut at: u_int = 0;
    let mut wrapped: libc::c_int = 1 as libc::c_int;
    if already == 0 {
        to = (*target).sy;
        gl = grid_reflow_move(target, &mut *((*gd).linedata).offset(yy as isize));
    } else {
        to = ((*target).sy).wrapping_sub(1 as libc::c_int as u_int);
        gl = &mut *((*target).linedata).offset(to as isize) as *mut grid_line;
    }
    at = (*gl).cellused;
    lines = 0 as libc::c_int as u_int;
    while !(yy.wrapping_add(lines) == ((*gd).hsize).wrapping_add((*gd).sy)) {
        line = yy.wrapping_add(1 as libc::c_int as u_int).wrapping_add(lines);
        if !(*((*gd).linedata).offset(line as isize)).flags & 0x1 as libc::c_int != 0 {
            wrapped = 0 as libc::c_int;
        }
        if (*((*gd).linedata).offset(line as isize)).cellused
            == 0 as libc::c_int as u_int
        {
            if wrapped == 0 {
                break;
            }
        } else {
            grid_get_cell1(
                &mut *((*gd).linedata).offset(line as isize),
                0 as libc::c_int as u_int,
                &mut gc,
            );
            if width.wrapping_add(gc.data.width as u_int) > sx {
                break;
            }
            width = width.wrapping_add(gc.data.width as u_int);
            grid_set_cell(target, at, to, &mut gc);
            at = at.wrapping_add(1);
            at;
            from = &mut *((*gd).linedata).offset(line as isize) as *mut grid_line;
            want = 1 as libc::c_int as u_int;
            while want < (*from).cellused {
                grid_get_cell1(from, want, &mut gc);
                if width.wrapping_add(gc.data.width as u_int) > sx {
                    break;
                }
                width = width.wrapping_add(gc.data.width as u_int);
                grid_set_cell(target, at, to, &mut gc);
                at = at.wrapping_add(1);
                at;
                want = want.wrapping_add(1);
                want;
            }
            lines = lines.wrapping_add(1);
            lines;
            if wrapped == 0 || want != (*from).cellused || width == sx {
                break;
            }
        }
    }
    if lines == 0 as libc::c_int as u_int {
        return;
    }
    left = ((*from).cellused).wrapping_sub(want);
    if left != 0 as libc::c_int as u_int {
        grid_move_cells(
            gd,
            0 as libc::c_int as u_int,
            want,
            yy.wrapping_add(lines),
            left,
            8 as libc::c_int as u_int,
        );
        (*from).cellused = left;
        (*from).cellsize = (*from).cellused;
        lines = lines.wrapping_sub(1);
        lines;
    } else if wrapped == 0 {
        (*gl).flags &= !(0x1 as libc::c_int);
    }
    i = yy.wrapping_add(1 as libc::c_int as u_int);
    while i < yy.wrapping_add(1 as libc::c_int as u_int).wrapping_add(lines) {
        free((*((*gd).linedata).offset(i as isize)).celldata as *mut libc::c_void);
        free((*((*gd).linedata).offset(i as isize)).extddata as *mut libc::c_void);
        grid_reflow_dead(&mut *((*gd).linedata).offset(i as isize));
        i = i.wrapping_add(1);
        i;
    }
    if *cy > to.wrapping_add(lines) {
        *cy = (*cy).wrapping_sub(lines);
    } else if *cy > to {
        *cy = to;
    }
    if (*gd).hscrolled > to.wrapping_add(lines) {
        (*gd).hscrolled = ((*gd).hscrolled).wrapping_sub(lines);
    } else if (*gd).hscrolled > to {
        (*gd).hscrolled = to;
    }
}
unsafe extern "C" fn grid_reflow_split(
    mut target: *mut grid,
    mut gd: *mut grid,
    mut sx: u_int,
    mut yy: u_int,
    mut at: u_int,
    mut cy: *mut u_int,
) {
    let mut gl: *mut grid_line = &mut *((*gd).linedata).offset(yy as isize)
        as *mut grid_line;
    let mut first: *mut grid_line = 0 as *mut grid_line;
    let mut gc: grid_cell = grid_cell {
        flags: 0,
        attr: 0,
        fg: 0,
        bg: 0,
        data: utf8_data {
            data: [0; 9],
            have: 0,
            size: 0,
            width: 0,
        },
    };
    let mut line: u_int = 0;
    let mut lines: u_int = 0;
    let mut width: u_int = 0;
    let mut i: u_int = 0;
    let mut xx: u_int = 0;
    let mut used: u_int = (*gl).cellused;
    let mut flags: libc::c_int = (*gl).flags;
    if !(*gl).flags & 0x2 as libc::c_int != 0 {
        lines = (1 as libc::c_int as u_int)
            .wrapping_add(((*gl).cellused).wrapping_sub(1 as libc::c_int as u_int) / sx);
    } else {
        lines = 2 as libc::c_int as u_int;
        width = 0 as libc::c_int as u_int;
        i = at;
        while i < used {
            grid_get_cell1(gl, i, &mut gc);
            if width.wrapping_add(gc.data.width as u_int) > sx {
                lines = lines.wrapping_add(1);
                lines;
                width = 0 as libc::c_int as u_int;
            }
            width = width.wrapping_add(gc.data.width as u_int);
            i = i.wrapping_add(1);
            i;
        }
    }
    line = ((*target).sy).wrapping_add(1 as libc::c_int as u_int);
    first = grid_reflow_add(target, lines);
    width = 0 as libc::c_int as u_int;
    xx = 0 as libc::c_int as u_int;
    i = at;
    while i < used {
        grid_get_cell1(gl, i, &mut gc);
        if width.wrapping_add(gc.data.width as u_int) > sx {
            (*((*target).linedata).offset(line as isize)).flags |= 0x1 as libc::c_int;
            line = line.wrapping_add(1);
            line;
            width = 0 as libc::c_int as u_int;
            xx = 0 as libc::c_int as u_int;
        }
        width = width.wrapping_add(gc.data.width as u_int);
        grid_set_cell(target, xx, line, &mut gc);
        xx = xx.wrapping_add(1);
        xx;
        i = i.wrapping_add(1);
        i;
    }
    if flags & 0x1 as libc::c_int != 0 {
        (*((*target).linedata).offset(line as isize)).flags |= 0x1 as libc::c_int;
    }
    (*gl).cellused = at;
    (*gl).cellsize = (*gl).cellused;
    (*gl).flags |= 0x1 as libc::c_int;
    memcpy(
        first as *mut libc::c_void,
        gl as *const libc::c_void,
        ::core::mem::size_of::<grid_line>() as libc::c_ulong,
    );
    grid_reflow_dead(gl);
    if yy <= *cy {
        *cy = (*cy).wrapping_add(lines.wrapping_sub(1 as libc::c_int as u_int));
    }
    if yy <= (*gd).hscrolled {
        (*gd)
            .hscrolled = ((*gd).hscrolled)
            .wrapping_add(lines.wrapping_sub(1 as libc::c_int as u_int));
    }
    if width < sx && flags & 0x1 as libc::c_int != 0 {
        grid_reflow_join(target, gd, sx, yy, width, cy, 1 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn grid_reflow(
    mut gd: *mut grid,
    mut sx: u_int,
    mut cursor: *mut u_int,
) {
    let mut target: *mut grid = 0 as *mut grid;
    let mut gl: *mut grid_line = 0 as *mut grid_line;
    let mut gc: grid_cell = grid_cell {
        flags: 0,
        attr: 0,
        fg: 0,
        bg: 0,
        data: utf8_data {
            data: [0; 9],
            have: 0,
            size: 0,
            width: 0,
        },
    };
    let mut yy: u_int = 0;
    let mut cy: u_int = 0;
    let mut width: u_int = 0;
    let mut i: u_int = 0;
    let mut at: u_int = 0;
    let mut first: u_int = 0;
    let mut start: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    gettimeofday(&mut start, 0 as *mut libc::c_void);
    log_debug(
        b"%s: %u lines, new width %u\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"grid_reflow\0"))
            .as_ptr(),
        ((*gd).hsize).wrapping_add((*gd).sy),
        sx,
    );
    cy = ((*gd).hsize).wrapping_add(*cursor);
    target = grid_create((*gd).sx, 0 as libc::c_int as u_int, 0 as libc::c_int as u_int);
    yy = 0 as libc::c_int as u_int;
    while yy < ((*gd).hsize).wrapping_add((*gd).sy) {
        gl = &mut *((*gd).linedata).offset(yy as isize) as *mut grid_line;
        if !((*gl).flags & 0x4 as libc::c_int != 0) {
            width = 0 as libc::c_int as u_int;
            at = width;
            first = at;
            if !(*gl).flags & 0x2 as libc::c_int != 0 {
                first = 1 as libc::c_int as u_int;
                width = (*gl).cellused;
                if width > sx {
                    at = sx;
                } else {
                    at = width;
                }
            } else {
                i = 0 as libc::c_int as u_int;
                while i < (*gl).cellused {
                    grid_get_cell1(gl, i, &mut gc);
                    if i == 0 as libc::c_int as u_int {
                        first = gc.data.width as u_int;
                    }
                    if at == 0 as libc::c_int as u_int
                        && width.wrapping_add(gc.data.width as u_int) > sx
                    {
                        at = i;
                    }
                    width = width.wrapping_add(gc.data.width as u_int);
                    i = i.wrapping_add(1);
                    i;
                }
            }
            if width == sx || first > sx {
                grid_reflow_move(target, gl);
            } else if width > sx {
                grid_reflow_split(target, gd, sx, yy, at, &mut cy);
            } else if (*gl).flags & 0x1 as libc::c_int != 0 {
                grid_reflow_join(target, gd, sx, yy, width, &mut cy, 0 as libc::c_int);
            } else {
                grid_reflow_move(target, gl);
            }
        }
        yy = yy.wrapping_add(1);
        yy;
    }
    if (*target).sy < (*gd).sy {
        grid_reflow_add(target, ((*gd).sy).wrapping_sub((*target).sy));
    }
    (*gd).hsize = ((*target).sy).wrapping_sub((*gd).sy);
    free((*gd).linedata as *mut libc::c_void);
    (*gd).linedata = (*target).linedata;
    free(target as *mut libc::c_void);
    if (*gd).hscrolled > (*gd).hsize {
        (*gd).hscrolled = (*gd).hsize;
    }
    if cy < (*gd).hsize {
        *cursor = 0 as libc::c_int as u_int;
    } else {
        *cursor = cy.wrapping_sub((*gd).hsize);
    }
    gettimeofday(&mut tv, 0 as *mut libc::c_void);
    tv.tv_sec = tv.tv_sec - start.tv_sec;
    tv.tv_usec = tv.tv_usec - start.tv_usec;
    if tv.tv_usec < 0 as libc::c_int as __suseconds_t {
        tv.tv_sec -= 1;
        tv.tv_sec;
        tv.tv_usec += 1000000 as libc::c_int as __suseconds_t;
    }
    log_debug(
        b"%s: now %u lines (in %llu.%06u seconds)\0" as *const u8 as *const libc::c_char,
        (*::core::mem::transmute::<&[u8; 12], &[libc::c_char; 12]>(b"grid_reflow\0"))
            .as_ptr(),
        ((*gd).hsize).wrapping_add((*gd).sy),
        tv.tv_sec as libc::c_ulonglong,
        tv.tv_usec as u_int,
    );
}
