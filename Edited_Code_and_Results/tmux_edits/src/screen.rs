use ::libc;
extern "C" {
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
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn grid_create(_: u_int, _: u_int, _: u_int) -> *mut grid;
    fn grid_destroy(_: *mut grid);
    fn grid_clear_lines(_: *mut grid, _: u_int, _: u_int, _: u_int);
    fn grid_reflow(_: *mut grid, _: u_int, _: *mut u_int);
    fn grid_view_delete_lines(_: *mut grid, _: u_int, _: u_int, _: u_int);
    fn utf8_copy(_: *mut utf8_data, _: *const utf8_data);
    fn utf8_stravis(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int;
    fn fatal(_: *const libc::c_char, _: ...) -> !;
    fn fatalx(_: *const libc::c_char, _: ...) -> !;
}
pub type __u_char = libc::c_uchar;
pub type __u_short = libc::c_ushort;
pub type __u_int = libc::c_uint;
pub type u_char = __u_char;
pub type u_short = __u_short;
pub type u_int = __u_int;
pub type size_t = libc::c_ulong;
pub type bitstr_t = libc::c_uchar;
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
pub struct screen {
    pub title: *mut libc::c_char,
    pub titles: *mut screen_titles,
    pub grid: *mut grid,
    pub cx: u_int,
    pub cy: u_int,
    pub cstyle: u_int,
    pub ccolour: *mut libc::c_char,
    pub rupper: u_int,
    pub rlower: u_int,
    pub mode: libc::c_int,
    pub tabs: *mut bitstr_t,
    pub sel: screen_sel,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct screen_sel {
    pub flag: libc::c_int,
    pub hidden: libc::c_int,
    pub rectflag: libc::c_int,
    pub lineflag: C2RustUnnamed,
    pub modekeys: libc::c_int,
    pub sx: u_int,
    pub sy: u_int,
    pub ex: u_int,
    pub ey: u_int,
    pub cell: grid_cell,
}
pub type C2RustUnnamed = libc::c_uint;
pub const LINE_SEL_RIGHT_LEFT: C2RustUnnamed = 2;
pub const LINE_SEL_LEFT_RIGHT: C2RustUnnamed = 1;
pub const LINE_SEL_NONE: C2RustUnnamed = 0;
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
    pub c2rust_unnamed: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
    pub offset: u_int,
    pub data: C2RustUnnamed_1,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_1 {
    pub attr: u_char,
    pub fg: u_char,
    pub bg: u_char,
    pub data: u_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct screen_titles {
    pub tqh_first: *mut screen_title_entry,
    pub tqh_last: *mut *mut screen_title_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct screen_title_entry {
    pub text: *mut libc::c_char,
    pub entry: C2RustUnnamed_2,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_2 {
    pub tqe_next: *mut screen_title_entry,
    pub tqe_prev: *mut *mut screen_title_entry,
}
unsafe extern "C" fn screen_free_titles(mut s: *mut screen) {
    let mut title_entry: *mut screen_title_entry = 0 as *mut screen_title_entry;
    if ((*s).titles).is_null() {
        return;
    }
    loop {
        title_entry = (*(*s).titles).tqh_first;
        if title_entry.is_null() {
            break;
        }
        if !((*title_entry).entry.tqe_next).is_null() {
            (*(*title_entry).entry.tqe_next)
                .entry
                .tqe_prev = (*title_entry).entry.tqe_prev;
        } else {
            (*(*s).titles).tqh_last = (*title_entry).entry.tqe_prev;
        }
        *(*title_entry).entry.tqe_prev = (*title_entry).entry.tqe_next;
        free((*title_entry).text as *mut libc::c_void);
        free(title_entry as *mut libc::c_void);
    }
    free((*s).titles as *mut libc::c_void);
    (*s).titles = 0 as *mut screen_titles;
}
#[no_mangle]
pub unsafe extern "C" fn screen_init(
    mut s: *mut screen,
    mut sx: u_int,
    mut sy: u_int,
    mut hlimit: u_int,
) {
    (*s).grid = grid_create(sx, sy, hlimit);
    (*s).title = xstrdup(b"\0" as *const u8 as *const libc::c_char);
    (*s).titles = 0 as *mut screen_titles;
    (*s).cstyle = 0 as libc::c_int as u_int;
    (*s).ccolour = xstrdup(b"\0" as *const u8 as *const libc::c_char);
    (*s).tabs = 0 as *mut bitstr_t;
    screen_reinit(s);
}
#[no_mangle]
pub unsafe extern "C" fn screen_reinit(mut s: *mut screen) {
    (*s).cx = 0 as libc::c_int as u_int;
    (*s).cy = 0 as libc::c_int as u_int;
    (*s).rupper = 0 as libc::c_int as u_int;
    (*s).rlower = ((*(*s).grid).sy).wrapping_sub(1 as libc::c_int as u_int);
    (*s).mode = 0x1 as libc::c_int | 0x10 as libc::c_int;
    screen_reset_tabs(s);
    grid_clear_lines(
        (*s).grid,
        (*(*s).grid).hsize,
        (*(*s).grid).sy,
        8 as libc::c_int as u_int,
    );
    screen_clear_selection(s);
    screen_free_titles(s);
}
#[no_mangle]
pub unsafe extern "C" fn screen_free(mut s: *mut screen) {
    free((*s).tabs as *mut libc::c_void);
    free((*s).title as *mut libc::c_void);
    free((*s).ccolour as *mut libc::c_void);
    grid_destroy((*s).grid);
    screen_free_titles(s);
}
#[no_mangle]
pub unsafe extern "C" fn screen_reset_tabs(mut s: *mut screen) {
    let mut i: u_int = 0;
    free((*s).tabs as *mut libc::c_void);
    (*s)
        .tabs = calloc(
        (((*(*s).grid).sx).wrapping_add(7 as libc::c_int as u_int) >> 3 as libc::c_int)
            as size_t,
        ::core::mem::size_of::<bitstr_t>() as libc::c_ulong,
    ) as *mut bitstr_t;
    if ((*s).tabs).is_null() {
        fatal(b"bit_alloc failed\0" as *const u8 as *const libc::c_char);
    }
    i = 8 as libc::c_int as u_int;
    while i < (*(*s).grid).sx {
        let ref mut fresh0 = *((*s).tabs).offset((i >> 3 as libc::c_int) as isize);
        *fresh0 = (*fresh0 as libc::c_int
            | (1 as libc::c_int) << (i & 0x7 as libc::c_int as u_int)) as bitstr_t;
        i = i.wrapping_add(8 as libc::c_int as u_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn screen_set_cursor_style(mut s: *mut screen, mut style: u_int) {
    if style <= 6 as libc::c_int as u_int {
        (*s).cstyle = style;
    }
}
#[no_mangle]
pub unsafe extern "C" fn screen_set_cursor_colour(
    mut s: *mut screen,
    mut colour: *const libc::c_char,
) {
    free((*s).ccolour as *mut libc::c_void);
    (*s).ccolour = xstrdup(colour);
}
#[no_mangle]
pub unsafe extern "C" fn screen_set_title(
    mut s: *mut screen,
    mut title: *const libc::c_char,
) {
    free((*s).title as *mut libc::c_void);
    utf8_stravis(
        &mut (*s).title,
        title,
        0x1 as libc::c_int | 0x2 as libc::c_int | 0x8 as libc::c_int
            | 0x10 as libc::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C" fn screen_push_title(mut s: *mut screen) {
    let mut title_entry: *mut screen_title_entry = 0 as *mut screen_title_entry;
    if ((*s).titles).is_null() {
        (*s)
            .titles = xmalloc(::core::mem::size_of::<screen_titles>() as libc::c_ulong)
            as *mut screen_titles;
        (*(*s).titles).tqh_first = 0 as *mut screen_title_entry;
        (*(*s).titles).tqh_last = &mut (*(*s).titles).tqh_first;
    }
    title_entry = xmalloc(::core::mem::size_of::<screen_title_entry>() as libc::c_ulong)
        as *mut screen_title_entry;
    (*title_entry).text = xstrdup((*s).title);
    (*title_entry).entry.tqe_next = (*(*s).titles).tqh_first;
    if !((*title_entry).entry.tqe_next).is_null() {
        (*(*(*s).titles).tqh_first).entry.tqe_prev = &mut (*title_entry).entry.tqe_next;
    } else {
        (*(*s).titles).tqh_last = &mut (*title_entry).entry.tqe_next;
    }
    (*(*s).titles).tqh_first = title_entry;
    (*title_entry).entry.tqe_prev = &mut (*(*s).titles).tqh_first;
}
#[no_mangle]
pub unsafe extern "C" fn screen_pop_title(mut s: *mut screen) {
    let mut title_entry: *mut screen_title_entry = 0 as *mut screen_title_entry;
    if ((*s).titles).is_null() {
        return;
    }
    title_entry = (*(*s).titles).tqh_first;
    if !title_entry.is_null() {
        screen_set_title(s, (*title_entry).text);
        if !((*title_entry).entry.tqe_next).is_null() {
            (*(*title_entry).entry.tqe_next)
                .entry
                .tqe_prev = (*title_entry).entry.tqe_prev;
        } else {
            (*(*s).titles).tqh_last = (*title_entry).entry.tqe_prev;
        }
        *(*title_entry).entry.tqe_prev = (*title_entry).entry.tqe_next;
        free((*title_entry).text as *mut libc::c_void);
        free(title_entry as *mut libc::c_void);
    }
}
#[no_mangle]
pub unsafe extern "C" fn screen_resize(
    mut s: *mut screen,
    mut sx: u_int,
    mut sy: u_int,
    mut reflow: libc::c_int,
) {
    if sx < 1 as libc::c_int as u_int {
        sx = 1 as libc::c_int as u_int;
    }
    if sy < 1 as libc::c_int as u_int {
        sy = 1 as libc::c_int as u_int;
    }
    if sx != (*(*s).grid).sx {
        screen_resize_x(s, sx);
        screen_reset_tabs(s);
    } else {
        reflow = 0 as libc::c_int;
    }
    if sy != (*(*s).grid).sy {
        screen_resize_y(s, sy);
    }
    if reflow != 0 {
        screen_reflow(s, sx);
    }
}
unsafe extern "C" fn screen_resize_x(mut s: *mut screen, mut sx: u_int) {
    let mut gd: *mut grid = (*s).grid;
    if sx == 0 as libc::c_int as u_int {
        fatalx(b"zero size\0" as *const u8 as *const libc::c_char);
    }
    if (*s).cx >= sx {
        (*s).cx = sx.wrapping_sub(1 as libc::c_int as u_int);
    }
    (*gd).sx = sx;
}
unsafe extern "C" fn screen_resize_y(mut s: *mut screen, mut sy: u_int) {
    let mut gd: *mut grid = (*s).grid;
    let mut needed: u_int = 0;
    let mut available: u_int = 0;
    let mut oldy: u_int = 0;
    let mut i: u_int = 0;
    if sy == 0 as libc::c_int as u_int {
        fatalx(b"zero size\0" as *const u8 as *const libc::c_char);
    }
    oldy = (*(*s).grid).sy;
    if sy < oldy {
        needed = oldy.wrapping_sub(sy);
        available = oldy.wrapping_sub(1 as libc::c_int as u_int).wrapping_sub((*s).cy);
        if available > 0 as libc::c_int as u_int {
            if available > needed {
                available = needed;
            }
            grid_view_delete_lines(
                gd,
                oldy.wrapping_sub(available),
                available,
                8 as libc::c_int as u_int,
            );
        }
        needed = needed.wrapping_sub(available);
        available = (*s).cy;
        if (*gd).flags & 0x1 as libc::c_int != 0 {
            (*gd).hscrolled = ((*gd).hscrolled).wrapping_add(needed);
            (*gd).hsize = ((*gd).hsize).wrapping_add(needed);
        } else if needed > 0 as libc::c_int as u_int
            && available > 0 as libc::c_int as u_int
        {
            if available > needed {
                available = needed;
            }
            grid_view_delete_lines(
                gd,
                0 as libc::c_int as u_int,
                available,
                8 as libc::c_int as u_int,
            );
        }
        (*s).cy = ((*s).cy).wrapping_sub(needed);
    }
    (*gd)
        .linedata = xreallocarray(
        (*gd).linedata as *mut libc::c_void,
        ((*gd).hsize).wrapping_add(sy) as size_t,
        ::core::mem::size_of::<grid_line>() as libc::c_ulong,
    ) as *mut grid_line;
    if sy > oldy {
        needed = sy.wrapping_sub(oldy);
        available = (*gd).hscrolled;
        if (*gd).flags & 0x1 as libc::c_int != 0 && available > 0 as libc::c_int as u_int
        {
            if available > needed {
                available = needed;
            }
            (*gd).hscrolled = ((*gd).hscrolled).wrapping_sub(available);
            (*gd).hsize = ((*gd).hsize).wrapping_sub(available);
            (*s).cy = ((*s).cy).wrapping_add(available);
        } else {
            available = 0 as libc::c_int as u_int;
        }
        needed = needed.wrapping_sub(available);
        i = ((*gd).hsize).wrapping_add(sy).wrapping_sub(needed);
        while i < ((*gd).hsize).wrapping_add(sy) {
            memset(
                &mut *((*gd).linedata).offset(i as isize) as *mut grid_line
                    as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<grid_line>() as libc::c_ulong,
            );
            i = i.wrapping_add(1);
            i;
        }
    }
    (*gd).sy = sy;
    (*s).rupper = 0 as libc::c_int as u_int;
    (*s).rlower = ((*(*s).grid).sy).wrapping_sub(1 as libc::c_int as u_int);
}
#[no_mangle]
pub unsafe extern "C" fn screen_set_selection(
    mut s: *mut screen,
    mut sx: u_int,
    mut sy: u_int,
    mut ex: u_int,
    mut ey: u_int,
    mut rectflag: u_int,
    mut gc: *mut grid_cell,
) {
    let mut sel: *mut screen_sel = &mut (*s).sel;
    memcpy(
        &mut (*sel).cell as *mut grid_cell as *mut libc::c_void,
        gc as *const libc::c_void,
        ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    (*sel).flag = 1 as libc::c_int;
    (*sel).hidden = 0 as libc::c_int;
    (*sel).rectflag = rectflag as libc::c_int;
    (*sel).sx = sx;
    (*sel).sy = sy;
    (*sel).ex = ex;
    (*sel).ey = ey;
}
#[no_mangle]
pub unsafe extern "C" fn screen_clear_selection(mut s: *mut screen) {
    let mut sel: *mut screen_sel = &mut (*s).sel;
    (*sel).flag = 0 as libc::c_int;
    (*sel).hidden = 0 as libc::c_int;
    (*sel).lineflag = LINE_SEL_NONE;
}
#[no_mangle]
pub unsafe extern "C" fn screen_hide_selection(mut s: *mut screen) {
    let mut sel: *mut screen_sel = &mut (*s).sel;
    (*sel).hidden = 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn screen_check_selection(
    mut s: *mut screen,
    mut px: u_int,
    mut py: u_int,
) -> libc::c_int {
    let mut sel: *mut screen_sel = &mut (*s).sel;
    let mut xx: u_int = 0;
    if (*sel).flag == 0 || (*sel).hidden != 0 {
        return 0 as libc::c_int;
    }
    if (*sel).rectflag != 0 {
        if (*sel).sy < (*sel).ey {
            if py < (*sel).sy || py > (*sel).ey {
                return 0 as libc::c_int;
            }
        } else if (*sel).sy > (*sel).ey {
            if py > (*sel).sy || py < (*sel).ey {
                return 0 as libc::c_int;
            }
        } else if py != (*sel).sy {
            return 0 as libc::c_int
        }
        if (*sel).ex < (*sel).sx {
            if px < (*sel).ex {
                return 0 as libc::c_int;
            }
            if px > (*sel).sx {
                return 0 as libc::c_int;
            }
        } else {
            if px < (*sel).sx {
                return 0 as libc::c_int;
            }
            if px > (*sel).ex {
                return 0 as libc::c_int;
            }
        }
    } else if (*sel).sy < (*sel).ey {
        if py < (*sel).sy || py > (*sel).ey {
            return 0 as libc::c_int;
        }
        if py == (*sel).sy && px < (*sel).sx {
            return 0 as libc::c_int;
        }
        if py == (*sel).ey && px > (*sel).ex {
            return 0 as libc::c_int;
        }
    } else if (*sel).sy > (*sel).ey {
        if py > (*sel).sy || py < (*sel).ey {
            return 0 as libc::c_int;
        }
        if py == (*sel).ey && px < (*sel).ex {
            return 0 as libc::c_int;
        }
        if (*sel).modekeys == 0 as libc::c_int {
            xx = ((*sel).sx).wrapping_sub(1 as libc::c_int as u_int);
        } else {
            xx = (*sel).sx;
        }
        if py == (*sel).sy && ((*sel).sx == 0 as libc::c_int as u_int || px > xx) {
            return 0 as libc::c_int;
        }
    } else {
        if py != (*sel).sy {
            return 0 as libc::c_int;
        }
        if (*sel).ex < (*sel).sx {
            if (*sel).modekeys == 0 as libc::c_int {
                xx = ((*sel).sx).wrapping_sub(1 as libc::c_int as u_int);
            } else {
                xx = (*sel).sx;
            }
            if px > xx || px < (*sel).ex {
                return 0 as libc::c_int;
            }
        } else if px < (*sel).sx || px > (*sel).ex {
            return 0 as libc::c_int
        }
    }
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn screen_select_cell(
    mut s: *mut screen,
    mut dst: *mut grid_cell,
    mut src: *const grid_cell,
) {
    if (*s).sel.flag == 0 || (*s).sel.hidden != 0 {
        return;
    }
    memcpy(
        dst as *mut libc::c_void,
        &mut (*s).sel.cell as *mut grid_cell as *const libc::c_void,
        ::core::mem::size_of::<grid_cell>() as libc::c_ulong,
    );
    utf8_copy(&mut (*dst).data, &(*src).data);
    (*dst).attr = ((*dst).attr as libc::c_int & !(0x80 as libc::c_int)) as u_short;
    (*dst)
        .attr = ((*dst).attr as libc::c_int
        | (*src).attr as libc::c_int & 0x80 as libc::c_int) as u_short;
    (*dst).flags = (*src).flags;
}
unsafe extern "C" fn screen_reflow(mut s: *mut screen, mut new_x: u_int) {
    grid_reflow((*s).grid, new_x, &mut (*s).cy);
}
