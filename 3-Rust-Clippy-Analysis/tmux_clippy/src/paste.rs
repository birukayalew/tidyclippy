use ::libc;
extern "C" {
    pub type options;
    fn free(_: *mut libc::c_void);
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strlcpy(
        __dest: *mut libc::c_char,
        __src: *const libc::c_char,
        __n: size_t,
    ) -> size_t;
    fn time(__timer: *mut time_t) -> time_t;
    fn xmalloc(_: size_t) -> *mut libc::c_void;
    fn xreallocarray(_: *mut libc::c_void, _: size_t, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn xasprintf(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    static mut global_options: *mut options;
    fn options_get_number(_: *mut options, _: *const libc::c_char) -> libc::c_longlong;
    fn utf8_strvis(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: size_t,
        _: libc::c_int,
    ) -> libc::c_int;
}
pub type __u_int = libc::c_uint;
pub type __time_t = libc::c_long;
pub type u_int = __u_int;
pub type time_t = __time_t;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct paste_buffer {
    pub data: *mut libc::c_char,
    pub size: size_t,
    pub name: *mut libc::c_char,
    pub created: time_t,
    pub automatic: libc::c_int,
    pub order: u_int,
    pub name_entry: C2RustUnnamed_0,
    pub time_entry: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub rbe_left: *mut paste_buffer,
    pub rbe_right: *mut paste_buffer,
    pub rbe_parent: *mut paste_buffer,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
    pub rbe_left: *mut paste_buffer,
    pub rbe_right: *mut paste_buffer,
    pub rbe_parent: *mut paste_buffer,
    pub rbe_color: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct paste_time_tree {
    pub rbh_root: *mut paste_buffer,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct paste_name_tree {
    pub rbh_root: *mut paste_buffer,
}
static mut paste_next_index: u_int = 0;
static mut paste_next_order: u_int = 0;
static mut paste_num_automatic: u_int = 0;
static mut paste_by_name: paste_name_tree = paste_name_tree {
    rbh_root: 0 as *const paste_buffer as *mut paste_buffer,
};
static mut paste_by_time: paste_time_tree = paste_time_tree {
    rbh_root: 0 as *const paste_buffer as *mut paste_buffer,
};
unsafe extern "C" fn paste_name_tree_RB_FIND(
    mut head: *mut paste_name_tree,
    mut elm: *mut paste_buffer,
) -> *mut paste_buffer {
    let mut tmp: *mut paste_buffer = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = paste_cmp_names(elm, tmp);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).name_entry.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).name_entry.rbe_right;
        } else {
            return tmp
        }
    }
    return 0 as *mut paste_buffer;
}
unsafe extern "C" fn paste_name_tree_RB_REMOVE_COLOR(
    mut head: *mut paste_name_tree,
    mut parent: *mut paste_buffer,
    mut elm: *mut paste_buffer,
) {
    let mut tmp: *mut paste_buffer = 0 as *mut paste_buffer;
    while (elm.is_null() || (*elm).name_entry.rbe_color == 0 as libc::c_int)
        && elm != (*head).rbh_root
    {
        if (*parent).name_entry.rbe_left == elm {
            tmp = (*parent).name_entry.rbe_right;
            if (*tmp).name_entry.rbe_color == 1 as libc::c_int {
                (*tmp).name_entry.rbe_color = 0 as libc::c_int;
                (*parent).name_entry.rbe_color = 1 as libc::c_int;
                tmp = (*parent).name_entry.rbe_right;
                (*parent).name_entry.rbe_right = (*tmp).name_entry.rbe_left;
                if !((*parent).name_entry.rbe_right).is_null() {
                    (*(*tmp).name_entry.rbe_left).name_entry.rbe_parent = parent;
                }
                (*tmp).name_entry.rbe_parent = (*parent).name_entry.rbe_parent;
                if !((*tmp).name_entry.rbe_parent).is_null() {
                    if parent == (*(*parent).name_entry.rbe_parent).name_entry.rbe_left {
                        (*(*parent).name_entry.rbe_parent).name_entry.rbe_left = tmp;
                    } else {
                        (*(*parent).name_entry.rbe_parent).name_entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).name_entry.rbe_left = parent;
                (*parent).name_entry.rbe_parent = tmp;
                !((*tmp).name_entry.rbe_parent).is_null();
                tmp = (*parent).name_entry.rbe_right;
            }
            if (((*tmp).name_entry.rbe_left).is_null()
                || (*(*tmp).name_entry.rbe_left).name_entry.rbe_color
                    == 0 as libc::c_int)
                && (((*tmp).name_entry.rbe_right).is_null()
                    || (*(*tmp).name_entry.rbe_right).name_entry.rbe_color
                        == 0 as libc::c_int)
            {
                (*tmp).name_entry.rbe_color = 1 as libc::c_int;
                elm = parent;
                parent = (*elm).name_entry.rbe_parent;
            } else {
                if ((*tmp).name_entry.rbe_right).is_null()
                    || (*(*tmp).name_entry.rbe_right).name_entry.rbe_color
                        == 0 as libc::c_int
                {
                    let mut oleft: *mut paste_buffer = 0 as *mut paste_buffer;
                    oleft = (*tmp).name_entry.rbe_left;
                    if !oleft.is_null() {
                        (*oleft).name_entry.rbe_color = 0 as libc::c_int;
                    }
                    (*tmp).name_entry.rbe_color = 1 as libc::c_int;
                    oleft = (*tmp).name_entry.rbe_left;
                    (*tmp).name_entry.rbe_left = (*oleft).name_entry.rbe_right;
                    if !((*tmp).name_entry.rbe_left).is_null() {
                        (*(*oleft).name_entry.rbe_right).name_entry.rbe_parent = tmp;
                    }
                    (*oleft).name_entry.rbe_parent = (*tmp).name_entry.rbe_parent;
                    if !((*oleft).name_entry.rbe_parent).is_null() {
                        if tmp == (*(*tmp).name_entry.rbe_parent).name_entry.rbe_left {
                            (*(*tmp).name_entry.rbe_parent).name_entry.rbe_left = oleft;
                        } else {
                            (*(*tmp).name_entry.rbe_parent).name_entry.rbe_right = oleft;
                        }
                    } else {
                        (*head).rbh_root = oleft;
                    }
                    (*oleft).name_entry.rbe_right = tmp;
                    (*tmp).name_entry.rbe_parent = oleft;
                    !((*oleft).name_entry.rbe_parent).is_null();
                    tmp = (*parent).name_entry.rbe_right;
                }
                (*tmp).name_entry.rbe_color = (*parent).name_entry.rbe_color;
                (*parent).name_entry.rbe_color = 0 as libc::c_int;
                if !((*tmp).name_entry.rbe_right).is_null() {
                    (*(*tmp).name_entry.rbe_right)
                        .name_entry
                        .rbe_color = 0 as libc::c_int;
                }
                tmp = (*parent).name_entry.rbe_right;
                (*parent).name_entry.rbe_right = (*tmp).name_entry.rbe_left;
                if !((*parent).name_entry.rbe_right).is_null() {
                    (*(*tmp).name_entry.rbe_left).name_entry.rbe_parent = parent;
                }
                (*tmp).name_entry.rbe_parent = (*parent).name_entry.rbe_parent;
                if !((*tmp).name_entry.rbe_parent).is_null() {
                    if parent == (*(*parent).name_entry.rbe_parent).name_entry.rbe_left {
                        (*(*parent).name_entry.rbe_parent).name_entry.rbe_left = tmp;
                    } else {
                        (*(*parent).name_entry.rbe_parent).name_entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).name_entry.rbe_left = parent;
                (*parent).name_entry.rbe_parent = tmp;
                !((*tmp).name_entry.rbe_parent).is_null();
                elm = (*head).rbh_root;
                break;
            }
        } else {
            tmp = (*parent).name_entry.rbe_left;
            if (*tmp).name_entry.rbe_color == 1 as libc::c_int {
                (*tmp).name_entry.rbe_color = 0 as libc::c_int;
                (*parent).name_entry.rbe_color = 1 as libc::c_int;
                tmp = (*parent).name_entry.rbe_left;
                (*parent).name_entry.rbe_left = (*tmp).name_entry.rbe_right;
                if !((*parent).name_entry.rbe_left).is_null() {
                    (*(*tmp).name_entry.rbe_right).name_entry.rbe_parent = parent;
                }
                (*tmp).name_entry.rbe_parent = (*parent).name_entry.rbe_parent;
                if !((*tmp).name_entry.rbe_parent).is_null() {
                    if parent == (*(*parent).name_entry.rbe_parent).name_entry.rbe_left {
                        (*(*parent).name_entry.rbe_parent).name_entry.rbe_left = tmp;
                    } else {
                        (*(*parent).name_entry.rbe_parent).name_entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).name_entry.rbe_right = parent;
                (*parent).name_entry.rbe_parent = tmp;
                !((*tmp).name_entry.rbe_parent).is_null();
                tmp = (*parent).name_entry.rbe_left;
            }
            if (((*tmp).name_entry.rbe_left).is_null()
                || (*(*tmp).name_entry.rbe_left).name_entry.rbe_color
                    == 0 as libc::c_int)
                && (((*tmp).name_entry.rbe_right).is_null()
                    || (*(*tmp).name_entry.rbe_right).name_entry.rbe_color
                        == 0 as libc::c_int)
            {
                (*tmp).name_entry.rbe_color = 1 as libc::c_int;
                elm = parent;
                parent = (*elm).name_entry.rbe_parent;
            } else {
                if ((*tmp).name_entry.rbe_left).is_null()
                    || (*(*tmp).name_entry.rbe_left).name_entry.rbe_color
                        == 0 as libc::c_int
                {
                    let mut oright: *mut paste_buffer = 0 as *mut paste_buffer;
                    oright = (*tmp).name_entry.rbe_right;
                    if !oright.is_null() {
                        (*oright).name_entry.rbe_color = 0 as libc::c_int;
                    }
                    (*tmp).name_entry.rbe_color = 1 as libc::c_int;
                    oright = (*tmp).name_entry.rbe_right;
                    (*tmp).name_entry.rbe_right = (*oright).name_entry.rbe_left;
                    if !((*tmp).name_entry.rbe_right).is_null() {
                        (*(*oright).name_entry.rbe_left).name_entry.rbe_parent = tmp;
                    }
                    (*oright).name_entry.rbe_parent = (*tmp).name_entry.rbe_parent;
                    if !((*oright).name_entry.rbe_parent).is_null() {
                        if tmp == (*(*tmp).name_entry.rbe_parent).name_entry.rbe_left {
                            (*(*tmp).name_entry.rbe_parent).name_entry.rbe_left = oright;
                        } else {
                            (*(*tmp).name_entry.rbe_parent)
                                .name_entry
                                .rbe_right = oright;
                        }
                    } else {
                        (*head).rbh_root = oright;
                    }
                    (*oright).name_entry.rbe_left = tmp;
                    (*tmp).name_entry.rbe_parent = oright;
                    !((*oright).name_entry.rbe_parent).is_null();
                    tmp = (*parent).name_entry.rbe_left;
                }
                (*tmp).name_entry.rbe_color = (*parent).name_entry.rbe_color;
                (*parent).name_entry.rbe_color = 0 as libc::c_int;
                if !((*tmp).name_entry.rbe_left).is_null() {
                    (*(*tmp).name_entry.rbe_left)
                        .name_entry
                        .rbe_color = 0 as libc::c_int;
                }
                tmp = (*parent).name_entry.rbe_left;
                (*parent).name_entry.rbe_left = (*tmp).name_entry.rbe_right;
                if !((*parent).name_entry.rbe_left).is_null() {
                    (*(*tmp).name_entry.rbe_right).name_entry.rbe_parent = parent;
                }
                (*tmp).name_entry.rbe_parent = (*parent).name_entry.rbe_parent;
                if !((*tmp).name_entry.rbe_parent).is_null() {
                    if parent == (*(*parent).name_entry.rbe_parent).name_entry.rbe_left {
                        (*(*parent).name_entry.rbe_parent).name_entry.rbe_left = tmp;
                    } else {
                        (*(*parent).name_entry.rbe_parent).name_entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).name_entry.rbe_right = parent;
                (*parent).name_entry.rbe_parent = tmp;
                !((*tmp).name_entry.rbe_parent).is_null();
                elm = (*head).rbh_root;
                break;
            }
        }
    }
    if !elm.is_null() {
        (*elm).name_entry.rbe_color = 0 as libc::c_int;
    }
}
unsafe extern "C" fn paste_name_tree_RB_INSERT(
    mut head: *mut paste_name_tree,
    mut elm: *mut paste_buffer,
) -> *mut paste_buffer {
    let mut tmp: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut parent: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut comp: libc::c_int = 0 as libc::c_int;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = paste_cmp_names(elm, parent);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).name_entry.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).name_entry.rbe_right;
        } else {
            return tmp
        }
    }
    (*elm).name_entry.rbe_parent = parent;
    (*elm).name_entry.rbe_right = 0 as *mut paste_buffer;
    (*elm).name_entry.rbe_left = (*elm).name_entry.rbe_right;
    (*elm).name_entry.rbe_color = 1 as libc::c_int;
    if !parent.is_null() {
        if comp < 0 as libc::c_int {
            (*parent).name_entry.rbe_left = elm;
        } else {
            (*parent).name_entry.rbe_right = elm;
        }
    } else {
        (*head).rbh_root = elm;
    }
    paste_name_tree_RB_INSERT_COLOR(head, elm);
    return 0 as *mut paste_buffer;
}
unsafe extern "C" fn paste_name_tree_RB_REMOVE(
    mut head: *mut paste_name_tree,
    mut elm: *mut paste_buffer,
) -> *mut paste_buffer {
    let mut current_block: u64;
    let mut child: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut parent: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut old: *mut paste_buffer = elm;
    let mut color: libc::c_int = 0;
    if ((*elm).name_entry.rbe_left).is_null() {
        child = (*elm).name_entry.rbe_right;
        current_block = 7245201122033322888;
    } else if ((*elm).name_entry.rbe_right).is_null() {
        child = (*elm).name_entry.rbe_left;
        current_block = 7245201122033322888;
    } else {
        let mut left: *mut paste_buffer = 0 as *mut paste_buffer;
        elm = (*elm).name_entry.rbe_right;
        loop {
            left = (*elm).name_entry.rbe_left;
            if left.is_null() {
                break;
            }
            elm = left;
        }
        child = (*elm).name_entry.rbe_right;
        parent = (*elm).name_entry.rbe_parent;
        color = (*elm).name_entry.rbe_color;
        if !child.is_null() {
            (*child).name_entry.rbe_parent = parent;
        }
        if !parent.is_null() {
            if (*parent).name_entry.rbe_left == elm {
                (*parent).name_entry.rbe_left = child;
            } else {
                (*parent).name_entry.rbe_right = child;
            }
        } else {
            (*head).rbh_root = child;
        }
        if (*elm).name_entry.rbe_parent == old {
            parent = elm;
        }
        (*elm).name_entry = (*old).name_entry;
        if !((*old).name_entry.rbe_parent).is_null() {
            if (*(*old).name_entry.rbe_parent).name_entry.rbe_left == old {
                (*(*old).name_entry.rbe_parent).name_entry.rbe_left = elm;
            } else {
                (*(*old).name_entry.rbe_parent).name_entry.rbe_right = elm;
            }
        } else {
            (*head).rbh_root = elm;
        }
        (*(*old).name_entry.rbe_left).name_entry.rbe_parent = elm;
        if !((*old).name_entry.rbe_right).is_null() {
            (*(*old).name_entry.rbe_right).name_entry.rbe_parent = elm;
        }
        if !parent.is_null() {
            left = parent;
            loop {
                left = (*left).name_entry.rbe_parent;
                if left.is_null() {
                    break;
                }
            }
        }
        current_block = 4351982961916307623;
    }
    match current_block {
        7245201122033322888 => {
            parent = (*elm).name_entry.rbe_parent;
            color = (*elm).name_entry.rbe_color;
            if !child.is_null() {
                (*child).name_entry.rbe_parent = parent;
            }
            if !parent.is_null() {
                if (*parent).name_entry.rbe_left == elm {
                    (*parent).name_entry.rbe_left = child;
                } else {
                    (*parent).name_entry.rbe_right = child;
                }
            } else {
                (*head).rbh_root = child;
            }
        }
        _ => {}
    }
    if color == 0 as libc::c_int {
        paste_name_tree_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
unsafe extern "C" fn paste_name_tree_RB_INSERT_COLOR(
    mut head: *mut paste_name_tree,
    mut elm: *mut paste_buffer,
) {
    let mut parent: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut gparent: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut tmp: *mut paste_buffer = 0 as *mut paste_buffer;
    loop {
        parent = (*elm).name_entry.rbe_parent;
        if !(!parent.is_null() && (*parent).name_entry.rbe_color == 1 as libc::c_int) {
            break;
        }
        gparent = (*parent).name_entry.rbe_parent;
        if parent == (*gparent).name_entry.rbe_left {
            tmp = (*gparent).name_entry.rbe_right;
            if !tmp.is_null() && (*tmp).name_entry.rbe_color == 1 as libc::c_int {
                (*tmp).name_entry.rbe_color = 0 as libc::c_int;
                (*parent).name_entry.rbe_color = 0 as libc::c_int;
                (*gparent).name_entry.rbe_color = 1 as libc::c_int;
                elm = gparent;
            } else {
                if (*parent).name_entry.rbe_right == elm {
                    tmp = (*parent).name_entry.rbe_right;
                    (*parent).name_entry.rbe_right = (*tmp).name_entry.rbe_left;
                    if !((*parent).name_entry.rbe_right).is_null() {
                        (*(*tmp).name_entry.rbe_left).name_entry.rbe_parent = parent;
                    }
                    (*tmp).name_entry.rbe_parent = (*parent).name_entry.rbe_parent;
                    if !((*tmp).name_entry.rbe_parent).is_null() {
                        if parent
                            == (*(*parent).name_entry.rbe_parent).name_entry.rbe_left
                        {
                            (*(*parent).name_entry.rbe_parent).name_entry.rbe_left = tmp;
                        } else {
                            (*(*parent).name_entry.rbe_parent)
                                .name_entry
                                .rbe_right = tmp;
                        }
                    } else {
                        (*head).rbh_root = tmp;
                    }
                    (*tmp).name_entry.rbe_left = parent;
                    (*parent).name_entry.rbe_parent = tmp;
                    !((*tmp).name_entry.rbe_parent).is_null();
                    tmp = parent;
                    parent = elm;
                    elm = tmp;
                }
                (*parent).name_entry.rbe_color = 0 as libc::c_int;
                (*gparent).name_entry.rbe_color = 1 as libc::c_int;
                tmp = (*gparent).name_entry.rbe_left;
                (*gparent).name_entry.rbe_left = (*tmp).name_entry.rbe_right;
                if !((*gparent).name_entry.rbe_left).is_null() {
                    (*(*tmp).name_entry.rbe_right).name_entry.rbe_parent = gparent;
                }
                (*tmp).name_entry.rbe_parent = (*gparent).name_entry.rbe_parent;
                if !((*tmp).name_entry.rbe_parent).is_null() {
                    if gparent == (*(*gparent).name_entry.rbe_parent).name_entry.rbe_left
                    {
                        (*(*gparent).name_entry.rbe_parent).name_entry.rbe_left = tmp;
                    } else {
                        (*(*gparent).name_entry.rbe_parent).name_entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).name_entry.rbe_right = gparent;
                (*gparent).name_entry.rbe_parent = tmp;
                !((*tmp).name_entry.rbe_parent).is_null();
            }
        } else {
            tmp = (*gparent).name_entry.rbe_left;
            if !tmp.is_null() && (*tmp).name_entry.rbe_color == 1 as libc::c_int {
                (*tmp).name_entry.rbe_color = 0 as libc::c_int;
                (*parent).name_entry.rbe_color = 0 as libc::c_int;
                (*gparent).name_entry.rbe_color = 1 as libc::c_int;
                elm = gparent;
            } else {
                if (*parent).name_entry.rbe_left == elm {
                    tmp = (*parent).name_entry.rbe_left;
                    (*parent).name_entry.rbe_left = (*tmp).name_entry.rbe_right;
                    if !((*parent).name_entry.rbe_left).is_null() {
                        (*(*tmp).name_entry.rbe_right).name_entry.rbe_parent = parent;
                    }
                    (*tmp).name_entry.rbe_parent = (*parent).name_entry.rbe_parent;
                    if !((*tmp).name_entry.rbe_parent).is_null() {
                        if parent
                            == (*(*parent).name_entry.rbe_parent).name_entry.rbe_left
                        {
                            (*(*parent).name_entry.rbe_parent).name_entry.rbe_left = tmp;
                        } else {
                            (*(*parent).name_entry.rbe_parent)
                                .name_entry
                                .rbe_right = tmp;
                        }
                    } else {
                        (*head).rbh_root = tmp;
                    }
                    (*tmp).name_entry.rbe_right = parent;
                    (*parent).name_entry.rbe_parent = tmp;
                    !((*tmp).name_entry.rbe_parent).is_null();
                    tmp = parent;
                    parent = elm;
                    elm = tmp;
                }
                (*parent).name_entry.rbe_color = 0 as libc::c_int;
                (*gparent).name_entry.rbe_color = 1 as libc::c_int;
                tmp = (*gparent).name_entry.rbe_right;
                (*gparent).name_entry.rbe_right = (*tmp).name_entry.rbe_left;
                if !((*gparent).name_entry.rbe_right).is_null() {
                    (*(*tmp).name_entry.rbe_left).name_entry.rbe_parent = gparent;
                }
                (*tmp).name_entry.rbe_parent = (*gparent).name_entry.rbe_parent;
                if !((*tmp).name_entry.rbe_parent).is_null() {
                    if gparent == (*(*gparent).name_entry.rbe_parent).name_entry.rbe_left
                    {
                        (*(*gparent).name_entry.rbe_parent).name_entry.rbe_left = tmp;
                    } else {
                        (*(*gparent).name_entry.rbe_parent).name_entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).name_entry.rbe_left = gparent;
                (*gparent).name_entry.rbe_parent = tmp;
                !((*tmp).name_entry.rbe_parent).is_null();
            }
        }
    }
    (*(*head).rbh_root).name_entry.rbe_color = 0 as libc::c_int;
}
unsafe extern "C" fn paste_time_tree_RB_PREV(
    mut elm: *mut paste_buffer,
) -> *mut paste_buffer {
    if !((*elm).time_entry.rbe_left).is_null() {
        elm = (*elm).time_entry.rbe_left;
        while !((*elm).time_entry.rbe_right).is_null() {
            elm = (*elm).time_entry.rbe_right;
        }
    } else if !((*elm).time_entry.rbe_parent).is_null()
        && elm == (*(*elm).time_entry.rbe_parent).time_entry.rbe_right
    {
        elm = (*elm).time_entry.rbe_parent;
    } else {
        while !((*elm).time_entry.rbe_parent).is_null()
            && elm == (*(*elm).time_entry.rbe_parent).time_entry.rbe_left
        {
            elm = (*elm).time_entry.rbe_parent;
        }
        elm = (*elm).time_entry.rbe_parent;
    }
    return elm;
}
unsafe extern "C" fn paste_time_tree_RB_NEXT(
    mut elm: *mut paste_buffer,
) -> *mut paste_buffer {
    if !((*elm).time_entry.rbe_right).is_null() {
        elm = (*elm).time_entry.rbe_right;
        while !((*elm).time_entry.rbe_left).is_null() {
            elm = (*elm).time_entry.rbe_left;
        }
    } else if !((*elm).time_entry.rbe_parent).is_null()
        && elm == (*(*elm).time_entry.rbe_parent).time_entry.rbe_left
    {
        elm = (*elm).time_entry.rbe_parent;
    } else {
        while !((*elm).time_entry.rbe_parent).is_null()
            && elm == (*(*elm).time_entry.rbe_parent).time_entry.rbe_right
        {
            elm = (*elm).time_entry.rbe_parent;
        }
        elm = (*elm).time_entry.rbe_parent;
    }
    return elm;
}
unsafe extern "C" fn paste_time_tree_RB_MINMAX(
    mut head: *mut paste_time_tree,
    mut val: libc::c_int,
) -> *mut paste_buffer {
    let mut tmp: *mut paste_buffer = (*head).rbh_root;
    let mut parent: *mut paste_buffer = 0 as *mut paste_buffer;
    while !tmp.is_null() {
        parent = tmp;
        if val < 0 as libc::c_int {
            tmp = (*tmp).time_entry.rbe_left;
        } else {
            tmp = (*tmp).time_entry.rbe_right;
        }
    }
    return parent;
}
unsafe extern "C" fn paste_time_tree_RB_INSERT_COLOR(
    mut head: *mut paste_time_tree,
    mut elm: *mut paste_buffer,
) {
    let mut parent: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut gparent: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut tmp: *mut paste_buffer = 0 as *mut paste_buffer;
    loop {
        parent = (*elm).time_entry.rbe_parent;
        if !(!parent.is_null() && (*parent).time_entry.rbe_color == 1 as libc::c_int) {
            break;
        }
        gparent = (*parent).time_entry.rbe_parent;
        if parent == (*gparent).time_entry.rbe_left {
            tmp = (*gparent).time_entry.rbe_right;
            if !tmp.is_null() && (*tmp).time_entry.rbe_color == 1 as libc::c_int {
                (*tmp).time_entry.rbe_color = 0 as libc::c_int;
                (*parent).time_entry.rbe_color = 0 as libc::c_int;
                (*gparent).time_entry.rbe_color = 1 as libc::c_int;
                elm = gparent;
            } else {
                if (*parent).time_entry.rbe_right == elm {
                    tmp = (*parent).time_entry.rbe_right;
                    (*parent).time_entry.rbe_right = (*tmp).time_entry.rbe_left;
                    if !((*parent).time_entry.rbe_right).is_null() {
                        (*(*tmp).time_entry.rbe_left).time_entry.rbe_parent = parent;
                    }
                    (*tmp).time_entry.rbe_parent = (*parent).time_entry.rbe_parent;
                    if !((*tmp).time_entry.rbe_parent).is_null() {
                        if parent
                            == (*(*parent).time_entry.rbe_parent).time_entry.rbe_left
                        {
                            (*(*parent).time_entry.rbe_parent).time_entry.rbe_left = tmp;
                        } else {
                            (*(*parent).time_entry.rbe_parent)
                                .time_entry
                                .rbe_right = tmp;
                        }
                    } else {
                        (*head).rbh_root = tmp;
                    }
                    (*tmp).time_entry.rbe_left = parent;
                    (*parent).time_entry.rbe_parent = tmp;
                    !((*tmp).time_entry.rbe_parent).is_null();
                    tmp = parent;
                    parent = elm;
                    elm = tmp;
                }
                (*parent).time_entry.rbe_color = 0 as libc::c_int;
                (*gparent).time_entry.rbe_color = 1 as libc::c_int;
                tmp = (*gparent).time_entry.rbe_left;
                (*gparent).time_entry.rbe_left = (*tmp).time_entry.rbe_right;
                if !((*gparent).time_entry.rbe_left).is_null() {
                    (*(*tmp).time_entry.rbe_right).time_entry.rbe_parent = gparent;
                }
                (*tmp).time_entry.rbe_parent = (*gparent).time_entry.rbe_parent;
                if !((*tmp).time_entry.rbe_parent).is_null() {
                    if gparent == (*(*gparent).time_entry.rbe_parent).time_entry.rbe_left
                    {
                        (*(*gparent).time_entry.rbe_parent).time_entry.rbe_left = tmp;
                    } else {
                        (*(*gparent).time_entry.rbe_parent).time_entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).time_entry.rbe_right = gparent;
                (*gparent).time_entry.rbe_parent = tmp;
                !((*tmp).time_entry.rbe_parent).is_null();
            }
        } else {
            tmp = (*gparent).time_entry.rbe_left;
            if !tmp.is_null() && (*tmp).time_entry.rbe_color == 1 as libc::c_int {
                (*tmp).time_entry.rbe_color = 0 as libc::c_int;
                (*parent).time_entry.rbe_color = 0 as libc::c_int;
                (*gparent).time_entry.rbe_color = 1 as libc::c_int;
                elm = gparent;
            } else {
                if (*parent).time_entry.rbe_left == elm {
                    tmp = (*parent).time_entry.rbe_left;
                    (*parent).time_entry.rbe_left = (*tmp).time_entry.rbe_right;
                    if !((*parent).time_entry.rbe_left).is_null() {
                        (*(*tmp).time_entry.rbe_right).time_entry.rbe_parent = parent;
                    }
                    (*tmp).time_entry.rbe_parent = (*parent).time_entry.rbe_parent;
                    if !((*tmp).time_entry.rbe_parent).is_null() {
                        if parent
                            == (*(*parent).time_entry.rbe_parent).time_entry.rbe_left
                        {
                            (*(*parent).time_entry.rbe_parent).time_entry.rbe_left = tmp;
                        } else {
                            (*(*parent).time_entry.rbe_parent)
                                .time_entry
                                .rbe_right = tmp;
                        }
                    } else {
                        (*head).rbh_root = tmp;
                    }
                    (*tmp).time_entry.rbe_right = parent;
                    (*parent).time_entry.rbe_parent = tmp;
                    !((*tmp).time_entry.rbe_parent).is_null();
                    tmp = parent;
                    parent = elm;
                    elm = tmp;
                }
                (*parent).time_entry.rbe_color = 0 as libc::c_int;
                (*gparent).time_entry.rbe_color = 1 as libc::c_int;
                tmp = (*gparent).time_entry.rbe_right;
                (*gparent).time_entry.rbe_right = (*tmp).time_entry.rbe_left;
                if !((*gparent).time_entry.rbe_right).is_null() {
                    (*(*tmp).time_entry.rbe_left).time_entry.rbe_parent = gparent;
                }
                (*tmp).time_entry.rbe_parent = (*gparent).time_entry.rbe_parent;
                if !((*tmp).time_entry.rbe_parent).is_null() {
                    if gparent == (*(*gparent).time_entry.rbe_parent).time_entry.rbe_left
                    {
                        (*(*gparent).time_entry.rbe_parent).time_entry.rbe_left = tmp;
                    } else {
                        (*(*gparent).time_entry.rbe_parent).time_entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).time_entry.rbe_left = gparent;
                (*gparent).time_entry.rbe_parent = tmp;
                !((*tmp).time_entry.rbe_parent).is_null();
            }
        }
    }
    (*(*head).rbh_root).time_entry.rbe_color = 0 as libc::c_int;
}
unsafe extern "C" fn paste_time_tree_RB_INSERT(
    mut head: *mut paste_time_tree,
    mut elm: *mut paste_buffer,
) -> *mut paste_buffer {
    let mut tmp: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut parent: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut comp: libc::c_int = 0 as libc::c_int;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = paste_cmp_times(elm, parent);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).time_entry.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).time_entry.rbe_right;
        } else {
            return tmp
        }
    }
    (*elm).time_entry.rbe_parent = parent;
    (*elm).time_entry.rbe_right = 0 as *mut paste_buffer;
    (*elm).time_entry.rbe_left = (*elm).time_entry.rbe_right;
    (*elm).time_entry.rbe_color = 1 as libc::c_int;
    if !parent.is_null() {
        if comp < 0 as libc::c_int {
            (*parent).time_entry.rbe_left = elm;
        } else {
            (*parent).time_entry.rbe_right = elm;
        }
    } else {
        (*head).rbh_root = elm;
    }
    paste_time_tree_RB_INSERT_COLOR(head, elm);
    return 0 as *mut paste_buffer;
}
unsafe extern "C" fn paste_time_tree_RB_REMOVE(
    mut head: *mut paste_time_tree,
    mut elm: *mut paste_buffer,
) -> *mut paste_buffer {
    let mut current_block: u64;
    let mut child: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut parent: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut old: *mut paste_buffer = elm;
    let mut color: libc::c_int = 0;
    if ((*elm).time_entry.rbe_left).is_null() {
        child = (*elm).time_entry.rbe_right;
        current_block = 7245201122033322888;
    } else if ((*elm).time_entry.rbe_right).is_null() {
        child = (*elm).time_entry.rbe_left;
        current_block = 7245201122033322888;
    } else {
        let mut left: *mut paste_buffer = 0 as *mut paste_buffer;
        elm = (*elm).time_entry.rbe_right;
        loop {
            left = (*elm).time_entry.rbe_left;
            if left.is_null() {
                break;
            }
            elm = left;
        }
        child = (*elm).time_entry.rbe_right;
        parent = (*elm).time_entry.rbe_parent;
        color = (*elm).time_entry.rbe_color;
        if !child.is_null() {
            (*child).time_entry.rbe_parent = parent;
        }
        if !parent.is_null() {
            if (*parent).time_entry.rbe_left == elm {
                (*parent).time_entry.rbe_left = child;
            } else {
                (*parent).time_entry.rbe_right = child;
            }
        } else {
            (*head).rbh_root = child;
        }
        if (*elm).time_entry.rbe_parent == old {
            parent = elm;
        }
        (*elm).time_entry = (*old).time_entry;
        if !((*old).time_entry.rbe_parent).is_null() {
            if (*(*old).time_entry.rbe_parent).time_entry.rbe_left == old {
                (*(*old).time_entry.rbe_parent).time_entry.rbe_left = elm;
            } else {
                (*(*old).time_entry.rbe_parent).time_entry.rbe_right = elm;
            }
        } else {
            (*head).rbh_root = elm;
        }
        (*(*old).time_entry.rbe_left).time_entry.rbe_parent = elm;
        if !((*old).time_entry.rbe_right).is_null() {
            (*(*old).time_entry.rbe_right).time_entry.rbe_parent = elm;
        }
        if !parent.is_null() {
            left = parent;
            loop {
                left = (*left).time_entry.rbe_parent;
                if left.is_null() {
                    break;
                }
            }
        }
        current_block = 8436278135197965483;
    }
    match current_block {
        7245201122033322888 => {
            parent = (*elm).time_entry.rbe_parent;
            color = (*elm).time_entry.rbe_color;
            if !child.is_null() {
                (*child).time_entry.rbe_parent = parent;
            }
            if !parent.is_null() {
                if (*parent).time_entry.rbe_left == elm {
                    (*parent).time_entry.rbe_left = child;
                } else {
                    (*parent).time_entry.rbe_right = child;
                }
            } else {
                (*head).rbh_root = child;
            }
        }
        _ => {}
    }
    if color == 0 as libc::c_int {
        paste_time_tree_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
unsafe extern "C" fn paste_time_tree_RB_REMOVE_COLOR(
    mut head: *mut paste_time_tree,
    mut parent: *mut paste_buffer,
    mut elm: *mut paste_buffer,
) {
    let mut tmp: *mut paste_buffer = 0 as *mut paste_buffer;
    while (elm.is_null() || (*elm).time_entry.rbe_color == 0 as libc::c_int)
        && elm != (*head).rbh_root
    {
        if (*parent).time_entry.rbe_left == elm {
            tmp = (*parent).time_entry.rbe_right;
            if (*tmp).time_entry.rbe_color == 1 as libc::c_int {
                (*tmp).time_entry.rbe_color = 0 as libc::c_int;
                (*parent).time_entry.rbe_color = 1 as libc::c_int;
                tmp = (*parent).time_entry.rbe_right;
                (*parent).time_entry.rbe_right = (*tmp).time_entry.rbe_left;
                if !((*parent).time_entry.rbe_right).is_null() {
                    (*(*tmp).time_entry.rbe_left).time_entry.rbe_parent = parent;
                }
                (*tmp).time_entry.rbe_parent = (*parent).time_entry.rbe_parent;
                if !((*tmp).time_entry.rbe_parent).is_null() {
                    if parent == (*(*parent).time_entry.rbe_parent).time_entry.rbe_left {
                        (*(*parent).time_entry.rbe_parent).time_entry.rbe_left = tmp;
                    } else {
                        (*(*parent).time_entry.rbe_parent).time_entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).time_entry.rbe_left = parent;
                (*parent).time_entry.rbe_parent = tmp;
                !((*tmp).time_entry.rbe_parent).is_null();
                tmp = (*parent).time_entry.rbe_right;
            }
            if (((*tmp).time_entry.rbe_left).is_null()
                || (*(*tmp).time_entry.rbe_left).time_entry.rbe_color
                    == 0 as libc::c_int)
                && (((*tmp).time_entry.rbe_right).is_null()
                    || (*(*tmp).time_entry.rbe_right).time_entry.rbe_color
                        == 0 as libc::c_int)
            {
                (*tmp).time_entry.rbe_color = 1 as libc::c_int;
                elm = parent;
                parent = (*elm).time_entry.rbe_parent;
            } else {
                if ((*tmp).time_entry.rbe_right).is_null()
                    || (*(*tmp).time_entry.rbe_right).time_entry.rbe_color
                        == 0 as libc::c_int
                {
                    let mut oleft: *mut paste_buffer = 0 as *mut paste_buffer;
                    oleft = (*tmp).time_entry.rbe_left;
                    if !oleft.is_null() {
                        (*oleft).time_entry.rbe_color = 0 as libc::c_int;
                    }
                    (*tmp).time_entry.rbe_color = 1 as libc::c_int;
                    oleft = (*tmp).time_entry.rbe_left;
                    (*tmp).time_entry.rbe_left = (*oleft).time_entry.rbe_right;
                    if !((*tmp).time_entry.rbe_left).is_null() {
                        (*(*oleft).time_entry.rbe_right).time_entry.rbe_parent = tmp;
                    }
                    (*oleft).time_entry.rbe_parent = (*tmp).time_entry.rbe_parent;
                    if !((*oleft).time_entry.rbe_parent).is_null() {
                        if tmp == (*(*tmp).time_entry.rbe_parent).time_entry.rbe_left {
                            (*(*tmp).time_entry.rbe_parent).time_entry.rbe_left = oleft;
                        } else {
                            (*(*tmp).time_entry.rbe_parent).time_entry.rbe_right = oleft;
                        }
                    } else {
                        (*head).rbh_root = oleft;
                    }
                    (*oleft).time_entry.rbe_right = tmp;
                    (*tmp).time_entry.rbe_parent = oleft;
                    !((*oleft).time_entry.rbe_parent).is_null();
                    tmp = (*parent).time_entry.rbe_right;
                }
                (*tmp).time_entry.rbe_color = (*parent).time_entry.rbe_color;
                (*parent).time_entry.rbe_color = 0 as libc::c_int;
                if !((*tmp).time_entry.rbe_right).is_null() {
                    (*(*tmp).time_entry.rbe_right)
                        .time_entry
                        .rbe_color = 0 as libc::c_int;
                }
                tmp = (*parent).time_entry.rbe_right;
                (*parent).time_entry.rbe_right = (*tmp).time_entry.rbe_left;
                if !((*parent).time_entry.rbe_right).is_null() {
                    (*(*tmp).time_entry.rbe_left).time_entry.rbe_parent = parent;
                }
                (*tmp).time_entry.rbe_parent = (*parent).time_entry.rbe_parent;
                if !((*tmp).time_entry.rbe_parent).is_null() {
                    if parent == (*(*parent).time_entry.rbe_parent).time_entry.rbe_left {
                        (*(*parent).time_entry.rbe_parent).time_entry.rbe_left = tmp;
                    } else {
                        (*(*parent).time_entry.rbe_parent).time_entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).time_entry.rbe_left = parent;
                (*parent).time_entry.rbe_parent = tmp;
                !((*tmp).time_entry.rbe_parent).is_null();
                elm = (*head).rbh_root;
                break;
            }
        } else {
            tmp = (*parent).time_entry.rbe_left;
            if (*tmp).time_entry.rbe_color == 1 as libc::c_int {
                (*tmp).time_entry.rbe_color = 0 as libc::c_int;
                (*parent).time_entry.rbe_color = 1 as libc::c_int;
                tmp = (*parent).time_entry.rbe_left;
                (*parent).time_entry.rbe_left = (*tmp).time_entry.rbe_right;
                if !((*parent).time_entry.rbe_left).is_null() {
                    (*(*tmp).time_entry.rbe_right).time_entry.rbe_parent = parent;
                }
                (*tmp).time_entry.rbe_parent = (*parent).time_entry.rbe_parent;
                if !((*tmp).time_entry.rbe_parent).is_null() {
                    if parent == (*(*parent).time_entry.rbe_parent).time_entry.rbe_left {
                        (*(*parent).time_entry.rbe_parent).time_entry.rbe_left = tmp;
                    } else {
                        (*(*parent).time_entry.rbe_parent).time_entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).time_entry.rbe_right = parent;
                (*parent).time_entry.rbe_parent = tmp;
                !((*tmp).time_entry.rbe_parent).is_null();
                tmp = (*parent).time_entry.rbe_left;
            }
            if (((*tmp).time_entry.rbe_left).is_null()
                || (*(*tmp).time_entry.rbe_left).time_entry.rbe_color
                    == 0 as libc::c_int)
                && (((*tmp).time_entry.rbe_right).is_null()
                    || (*(*tmp).time_entry.rbe_right).time_entry.rbe_color
                        == 0 as libc::c_int)
            {
                (*tmp).time_entry.rbe_color = 1 as libc::c_int;
                elm = parent;
                parent = (*elm).time_entry.rbe_parent;
            } else {
                if ((*tmp).time_entry.rbe_left).is_null()
                    || (*(*tmp).time_entry.rbe_left).time_entry.rbe_color
                        == 0 as libc::c_int
                {
                    let mut oright: *mut paste_buffer = 0 as *mut paste_buffer;
                    oright = (*tmp).time_entry.rbe_right;
                    if !oright.is_null() {
                        (*oright).time_entry.rbe_color = 0 as libc::c_int;
                    }
                    (*tmp).time_entry.rbe_color = 1 as libc::c_int;
                    oright = (*tmp).time_entry.rbe_right;
                    (*tmp).time_entry.rbe_right = (*oright).time_entry.rbe_left;
                    if !((*tmp).time_entry.rbe_right).is_null() {
                        (*(*oright).time_entry.rbe_left).time_entry.rbe_parent = tmp;
                    }
                    (*oright).time_entry.rbe_parent = (*tmp).time_entry.rbe_parent;
                    if !((*oright).time_entry.rbe_parent).is_null() {
                        if tmp == (*(*tmp).time_entry.rbe_parent).time_entry.rbe_left {
                            (*(*tmp).time_entry.rbe_parent).time_entry.rbe_left = oright;
                        } else {
                            (*(*tmp).time_entry.rbe_parent)
                                .time_entry
                                .rbe_right = oright;
                        }
                    } else {
                        (*head).rbh_root = oright;
                    }
                    (*oright).time_entry.rbe_left = tmp;
                    (*tmp).time_entry.rbe_parent = oright;
                    !((*oright).time_entry.rbe_parent).is_null();
                    tmp = (*parent).time_entry.rbe_left;
                }
                (*tmp).time_entry.rbe_color = (*parent).time_entry.rbe_color;
                (*parent).time_entry.rbe_color = 0 as libc::c_int;
                if !((*tmp).time_entry.rbe_left).is_null() {
                    (*(*tmp).time_entry.rbe_left)
                        .time_entry
                        .rbe_color = 0 as libc::c_int;
                }
                tmp = (*parent).time_entry.rbe_left;
                (*parent).time_entry.rbe_left = (*tmp).time_entry.rbe_right;
                if !((*parent).time_entry.rbe_left).is_null() {
                    (*(*tmp).time_entry.rbe_right).time_entry.rbe_parent = parent;
                }
                (*tmp).time_entry.rbe_parent = (*parent).time_entry.rbe_parent;
                if !((*tmp).time_entry.rbe_parent).is_null() {
                    if parent == (*(*parent).time_entry.rbe_parent).time_entry.rbe_left {
                        (*(*parent).time_entry.rbe_parent).time_entry.rbe_left = tmp;
                    } else {
                        (*(*parent).time_entry.rbe_parent).time_entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).time_entry.rbe_right = parent;
                (*parent).time_entry.rbe_parent = tmp;
                !((*tmp).time_entry.rbe_parent).is_null();
                elm = (*head).rbh_root;
                break;
            }
        }
    }
    if !elm.is_null() {
        (*elm).time_entry.rbe_color = 0 as libc::c_int;
    }
}
unsafe extern "C" fn paste_cmp_names(
    mut a: *const paste_buffer,
    mut b: *const paste_buffer,
) -> libc::c_int {
    return strcmp((*a).name, (*b).name);
}
unsafe extern "C" fn paste_cmp_times(
    mut a: *const paste_buffer,
    mut b: *const paste_buffer,
) -> libc::c_int {
    if (*a).order > (*b).order {
        return -(1 as libc::c_int);
    }
    if (*a).order < (*b).order {
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn paste_buffer_name(
    mut pb: *mut paste_buffer,
) -> *const libc::c_char {
    return (*pb).name;
}
#[no_mangle]
pub unsafe extern "C" fn paste_buffer_order(mut pb: *mut paste_buffer) -> u_int {
    return (*pb).order;
}
#[no_mangle]
pub unsafe extern "C" fn paste_buffer_created(mut pb: *mut paste_buffer) -> time_t {
    return (*pb).created;
}
#[no_mangle]
pub unsafe extern "C" fn paste_buffer_data(
    mut pb: *mut paste_buffer,
    mut size: *mut size_t,
) -> *const libc::c_char {
    if !size.is_null() {
        *size = (*pb).size;
    }
    return (*pb).data;
}
#[no_mangle]
pub unsafe extern "C" fn paste_walk(mut pb: *mut paste_buffer) -> *mut paste_buffer {
    if pb.is_null() {
        return paste_time_tree_RB_MINMAX(&mut paste_by_time, -(1 as libc::c_int));
    }
    return paste_time_tree_RB_NEXT(pb);
}
#[no_mangle]
pub unsafe extern "C" fn paste_get_top(
    mut name: *mut *const libc::c_char,
) -> *mut paste_buffer {
    let mut pb: *mut paste_buffer = 0 as *mut paste_buffer;
    pb = paste_time_tree_RB_MINMAX(&mut paste_by_time, -(1 as libc::c_int));
    if pb.is_null() {
        return 0 as *mut paste_buffer;
    }
    if !name.is_null() {
        *name = (*pb).name;
    }
    return pb;
}
#[no_mangle]
pub unsafe extern "C" fn paste_get_name(
    mut name: *const libc::c_char,
) -> *mut paste_buffer {
    let mut pbfind: paste_buffer = paste_buffer {
        data: 0 as *mut libc::c_char,
        size: 0,
        name: 0 as *mut libc::c_char,
        created: 0,
        automatic: 0,
        order: 0,
        name_entry: C2RustUnnamed_0 {
            rbe_left: 0 as *mut paste_buffer,
            rbe_right: 0 as *mut paste_buffer,
            rbe_parent: 0 as *mut paste_buffer,
            rbe_color: 0,
        },
        time_entry: C2RustUnnamed {
            rbe_left: 0 as *mut paste_buffer,
            rbe_right: 0 as *mut paste_buffer,
            rbe_parent: 0 as *mut paste_buffer,
            rbe_color: 0,
        },
    };
    if name.is_null() || *name as libc::c_int == '\0' as i32 {
        return 0 as *mut paste_buffer;
    }
    pbfind.name = name as *mut libc::c_char;
    return paste_name_tree_RB_FIND(&mut paste_by_name, &mut pbfind);
}
#[no_mangle]
pub unsafe extern "C" fn paste_free(mut pb: *mut paste_buffer) {
    paste_name_tree_RB_REMOVE(&mut paste_by_name, pb);
    paste_time_tree_RB_REMOVE(&mut paste_by_time, pb);
    if (*pb).automatic != 0 {
        paste_num_automatic = paste_num_automatic.wrapping_sub(1);
        paste_num_automatic;
    }
    free((*pb).data as *mut libc::c_void);
    free((*pb).name as *mut libc::c_void);
    free(pb as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn paste_add(mut data: *mut libc::c_char, mut size: size_t) {
    let mut pb: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut pb1: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut limit: u_int = 0;
    if size == 0 as libc::c_int as size_t {
        free(data as *mut libc::c_void);
        return;
    }
    limit = options_get_number(
        global_options,
        b"buffer-limit\0" as *const u8 as *const libc::c_char,
    ) as u_int;
    pb = paste_time_tree_RB_MINMAX(&mut paste_by_time, 1 as libc::c_int);
    while !pb.is_null()
        && {
            pb1 = paste_time_tree_RB_PREV(pb);
            1 as libc::c_int != 0
        }
    {
        if paste_num_automatic < limit {
            break;
        }
        if (*pb).automatic != 0 {
            paste_free(pb);
        }
        pb = pb1;
    }
    pb = xmalloc(::core::mem::size_of::<paste_buffer>() as libc::c_ulong)
        as *mut paste_buffer;
    (*pb).name = 0 as *mut libc::c_char;
    loop {
        free((*pb).name as *mut libc::c_void);
        xasprintf(
            &mut (*pb).name as *mut *mut libc::c_char,
            b"buffer%04u\0" as *const u8 as *const libc::c_char,
            paste_next_index,
        );
        paste_next_index = paste_next_index.wrapping_add(1);
        paste_next_index;
        if (paste_get_name((*pb).name)).is_null() {
            break;
        }
    }
    (*pb).data = data;
    (*pb).size = size;
    (*pb).automatic = 1 as libc::c_int;
    paste_num_automatic = paste_num_automatic.wrapping_add(1);
    paste_num_automatic;
    (*pb).created = time(0 as *mut time_t);
    let fresh0 = paste_next_order;
    paste_next_order = paste_next_order.wrapping_add(1);
    (*pb).order = fresh0;
    paste_name_tree_RB_INSERT(&mut paste_by_name, pb);
    paste_time_tree_RB_INSERT(&mut paste_by_time, pb);
}
#[no_mangle]
pub unsafe extern "C" fn paste_rename(
    mut oldname: *const libc::c_char,
    mut newname: *const libc::c_char,
    mut cause: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut pb: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut pb_new: *mut paste_buffer = 0 as *mut paste_buffer;
    if !cause.is_null() {
        *cause = 0 as *mut libc::c_char;
    }
    if oldname.is_null() || *oldname as libc::c_int == '\0' as i32 {
        if !cause.is_null() {
            *cause = xstrdup(b"no buffer\0" as *const u8 as *const libc::c_char);
        }
        return -(1 as libc::c_int);
    }
    if newname.is_null() || *newname as libc::c_int == '\0' as i32 {
        if !cause.is_null() {
            *cause = xstrdup(b"new name is empty\0" as *const u8 as *const libc::c_char);
        }
        return -(1 as libc::c_int);
    }
    pb = paste_get_name(oldname);
    if pb.is_null() {
        if !cause.is_null() {
            xasprintf(
                cause,
                b"no buffer %s\0" as *const u8 as *const libc::c_char,
                oldname,
            );
        }
        return -(1 as libc::c_int);
    }
    pb_new = paste_get_name(newname);
    if !pb_new.is_null() {
        if !cause.is_null() {
            xasprintf(
                cause,
                b"buffer %s already exists\0" as *const u8 as *const libc::c_char,
                newname,
            );
        }
        return -(1 as libc::c_int);
    }
    paste_name_tree_RB_REMOVE(&mut paste_by_name, pb);
    free((*pb).name as *mut libc::c_void);
    (*pb).name = xstrdup(newname);
    if (*pb).automatic != 0 {
        paste_num_automatic = paste_num_automatic.wrapping_sub(1);
        paste_num_automatic;
    }
    (*pb).automatic = 0 as libc::c_int;
    paste_name_tree_RB_INSERT(&mut paste_by_name, pb);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn paste_set(
    mut data: *mut libc::c_char,
    mut size: size_t,
    mut name: *const libc::c_char,
    mut cause: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut pb: *mut paste_buffer = 0 as *mut paste_buffer;
    let mut old: *mut paste_buffer = 0 as *mut paste_buffer;
    if !cause.is_null() {
        *cause = 0 as *mut libc::c_char;
    }
    if size == 0 as libc::c_int as size_t {
        free(data as *mut libc::c_void);
        return 0 as libc::c_int;
    }
    if name.is_null() {
        paste_add(data, size);
        return 0 as libc::c_int;
    }
    if *name as libc::c_int == '\0' as i32 {
        if !cause.is_null() {
            *cause = xstrdup(b"empty buffer name\0" as *const u8 as *const libc::c_char);
        }
        return -(1 as libc::c_int);
    }
    pb = xmalloc(::core::mem::size_of::<paste_buffer>() as libc::c_ulong)
        as *mut paste_buffer;
    (*pb).name = xstrdup(name);
    (*pb).data = data;
    (*pb).size = size;
    (*pb).automatic = 0 as libc::c_int;
    let fresh1 = paste_next_order;
    paste_next_order = paste_next_order.wrapping_add(1);
    (*pb).order = fresh1;
    (*pb).created = time(0 as *mut time_t);
    old = paste_get_name(name);
    if !old.is_null() {
        paste_free(old);
    }
    paste_name_tree_RB_INSERT(&mut paste_by_name, pb);
    paste_time_tree_RB_INSERT(&mut paste_by_time, pb);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn paste_make_sample(
    mut pb: *mut paste_buffer,
) -> *mut libc::c_char {
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: size_t = 0;
    let mut used: size_t = 0;
    let flags: libc::c_int = 0x1 as libc::c_int | 0x8 as libc::c_int
        | 0x10 as libc::c_int;
    let width: size_t = 200 as libc::c_int as size_t;
    len = (*pb).size;
    if len > width {
        len = width;
    }
    buf = xreallocarray(
        0 as *mut libc::c_void,
        len,
        (4 as libc::c_int + 4 as libc::c_int) as size_t,
    ) as *mut libc::c_char;
    used = utf8_strvis(buf, (*pb).data, len, flags) as size_t;
    if (*pb).size > width || used > width {
        strlcpy(
            buf.offset(width as isize),
            b"...\0" as *const u8 as *const libc::c_char,
            4 as libc::c_int as size_t,
        );
    }
    return buf;
}
