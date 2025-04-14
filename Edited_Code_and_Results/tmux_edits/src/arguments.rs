use ::libc;
extern "C" {
    fn free(_: *mut libc::c_void);
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strcspn(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_ulong;
    fn strtonum(
        _: *const libc::c_char,
        _: libc::c_longlong,
        _: libc::c_longlong,
        _: *mut *const libc::c_char,
    ) -> libc::c_longlong;
    static mut BSDoptind: libc::c_int;
    static mut BSDoptreset: libc::c_int;
    static mut BSDoptarg: *mut libc::c_char;
    fn BSDgetopt(
        _: libc::c_int,
        _: *const *mut libc::c_char,
        _: *const libc::c_char,
    ) -> libc::c_int;
    fn xcalloc(_: size_t, _: size_t) -> *mut libc::c_void;
    fn xstrdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn cmd_copy_argv(
        _: libc::c_int,
        _: *mut *mut libc::c_char,
    ) -> *mut *mut libc::c_char;
    fn cmd_free_argv(_: libc::c_int, _: *mut *mut libc::c_char);
    fn utf8_stravis(
        _: *mut *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_int,
    ) -> libc::c_int;
    fn args_print_add(
        buf: *mut *mut libc::c_char,
        len: *mut size_t,
        fmt: *const libc::c_char,
        _: ...
    );
}
pub type __u_char = libc::c_uchar;
pub type u_char = __u_char;
pub type size_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct args {
    pub tree: args_tree,
    pub argc: libc::c_int,
    pub argv: *mut *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct args_tree {
    pub rbh_root: *mut args_entry,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct args_entry {
    pub flag: u_char,
    pub value: *mut libc::c_char,
    pub entry: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub rbe_left: *mut args_entry,
    pub rbe_right: *mut args_entry,
    pub rbe_parent: *mut args_entry,
    pub rbe_color: libc::c_int,
}
unsafe extern "C" fn args_tree_RB_INSERT_COLOR(
    mut head: *mut args_tree,
    mut elm: *mut args_entry,
) {
    let mut parent: *mut args_entry = 0 as *mut args_entry;
    let mut gparent: *mut args_entry = 0 as *mut args_entry;
    let mut tmp: *mut args_entry = 0 as *mut args_entry;
    loop {
        parent = (*elm).entry.rbe_parent;
        if !(!parent.is_null() && (*parent).entry.rbe_color == 1 as libc::c_int) {
            break;
        }
        gparent = (*parent).entry.rbe_parent;
        if parent == (*gparent).entry.rbe_left {
            tmp = (*gparent).entry.rbe_right;
            if !tmp.is_null() && (*tmp).entry.rbe_color == 1 as libc::c_int {
                (*tmp).entry.rbe_color = 0 as libc::c_int;
                (*parent).entry.rbe_color = 0 as libc::c_int;
                (*gparent).entry.rbe_color = 1 as libc::c_int;
                elm = gparent;
            } else {
                if (*parent).entry.rbe_right == elm {
                    tmp = (*parent).entry.rbe_right;
                    (*parent).entry.rbe_right = (*tmp).entry.rbe_left;
                    if !((*parent).entry.rbe_right).is_null() {
                        (*(*tmp).entry.rbe_left).entry.rbe_parent = parent;
                    }
                    (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                    if !((*tmp).entry.rbe_parent).is_null() {
                        if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                            (*(*parent).entry.rbe_parent).entry.rbe_left = tmp;
                        } else {
                            (*(*parent).entry.rbe_parent).entry.rbe_right = tmp;
                        }
                    } else {
                        (*head).rbh_root = tmp;
                    }
                    (*tmp).entry.rbe_left = parent;
                    (*parent).entry.rbe_parent = tmp;
                    !((*tmp).entry.rbe_parent).is_null();
                    tmp = parent;
                    parent = elm;
                    elm = tmp;
                }
                (*parent).entry.rbe_color = 0 as libc::c_int;
                (*gparent).entry.rbe_color = 1 as libc::c_int;
                tmp = (*gparent).entry.rbe_left;
                (*gparent).entry.rbe_left = (*tmp).entry.rbe_right;
                if !((*gparent).entry.rbe_left).is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_parent = gparent;
                }
                (*tmp).entry.rbe_parent = (*gparent).entry.rbe_parent;
                if !((*tmp).entry.rbe_parent).is_null() {
                    if gparent == (*(*gparent).entry.rbe_parent).entry.rbe_left {
                        (*(*gparent).entry.rbe_parent).entry.rbe_left = tmp;
                    } else {
                        (*(*gparent).entry.rbe_parent).entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).entry.rbe_right = gparent;
                (*gparent).entry.rbe_parent = tmp;
                !((*tmp).entry.rbe_parent).is_null();
            }
        } else {
            tmp = (*gparent).entry.rbe_left;
            if !tmp.is_null() && (*tmp).entry.rbe_color == 1 as libc::c_int {
                (*tmp).entry.rbe_color = 0 as libc::c_int;
                (*parent).entry.rbe_color = 0 as libc::c_int;
                (*gparent).entry.rbe_color = 1 as libc::c_int;
                elm = gparent;
            } else {
                if (*parent).entry.rbe_left == elm {
                    tmp = (*parent).entry.rbe_left;
                    (*parent).entry.rbe_left = (*tmp).entry.rbe_right;
                    if !((*parent).entry.rbe_left).is_null() {
                        (*(*tmp).entry.rbe_right).entry.rbe_parent = parent;
                    }
                    (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                    if !((*tmp).entry.rbe_parent).is_null() {
                        if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                            (*(*parent).entry.rbe_parent).entry.rbe_left = tmp;
                        } else {
                            (*(*parent).entry.rbe_parent).entry.rbe_right = tmp;
                        }
                    } else {
                        (*head).rbh_root = tmp;
                    }
                    (*tmp).entry.rbe_right = parent;
                    (*parent).entry.rbe_parent = tmp;
                    !((*tmp).entry.rbe_parent).is_null();
                    tmp = parent;
                    parent = elm;
                    elm = tmp;
                }
                (*parent).entry.rbe_color = 0 as libc::c_int;
                (*gparent).entry.rbe_color = 1 as libc::c_int;
                tmp = (*gparent).entry.rbe_right;
                (*gparent).entry.rbe_right = (*tmp).entry.rbe_left;
                if !((*gparent).entry.rbe_right).is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_parent = gparent;
                }
                (*tmp).entry.rbe_parent = (*gparent).entry.rbe_parent;
                if !((*tmp).entry.rbe_parent).is_null() {
                    if gparent == (*(*gparent).entry.rbe_parent).entry.rbe_left {
                        (*(*gparent).entry.rbe_parent).entry.rbe_left = tmp;
                    } else {
                        (*(*gparent).entry.rbe_parent).entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).entry.rbe_left = gparent;
                (*gparent).entry.rbe_parent = tmp;
                !((*tmp).entry.rbe_parent).is_null();
            }
        }
    }
    (*(*head).rbh_root).entry.rbe_color = 0 as libc::c_int;
}
unsafe extern "C" fn args_tree_RB_FIND(
    mut head: *mut args_tree,
    mut elm: *mut args_entry,
) -> *mut args_entry {
    let mut tmp: *mut args_entry = (*head).rbh_root;
    let mut comp: libc::c_int = 0;
    while !tmp.is_null() {
        comp = args_cmp(elm, tmp);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right;
        } else {
            return tmp
        }
    }
    return 0 as *mut args_entry;
}
unsafe extern "C" fn args_tree_RB_REMOVE(
    mut head: *mut args_tree,
    mut elm: *mut args_entry,
) -> *mut args_entry {
    let mut current_block: u64;
    let mut child: *mut args_entry = 0 as *mut args_entry;
    let mut parent: *mut args_entry = 0 as *mut args_entry;
    let mut old: *mut args_entry = elm;
    let mut color: libc::c_int = 0;
    if ((*elm).entry.rbe_left).is_null() {
        child = (*elm).entry.rbe_right;
        current_block = 7245201122033322888;
    } else if ((*elm).entry.rbe_right).is_null() {
        child = (*elm).entry.rbe_left;
        current_block = 7245201122033322888;
    } else {
        let mut left: *mut args_entry = 0 as *mut args_entry;
        elm = (*elm).entry.rbe_right;
        loop {
            left = (*elm).entry.rbe_left;
            if left.is_null() {
                break;
            }
            elm = left;
        }
        child = (*elm).entry.rbe_right;
        parent = (*elm).entry.rbe_parent;
        color = (*elm).entry.rbe_color;
        if !child.is_null() {
            (*child).entry.rbe_parent = parent;
        }
        if !parent.is_null() {
            if (*parent).entry.rbe_left == elm {
                (*parent).entry.rbe_left = child;
            } else {
                (*parent).entry.rbe_right = child;
            }
        } else {
            (*head).rbh_root = child;
        }
        if (*elm).entry.rbe_parent == old {
            parent = elm;
        }
        (*elm).entry = (*old).entry;
        if !((*old).entry.rbe_parent).is_null() {
            if (*(*old).entry.rbe_parent).entry.rbe_left == old {
                (*(*old).entry.rbe_parent).entry.rbe_left = elm;
            } else {
                (*(*old).entry.rbe_parent).entry.rbe_right = elm;
            }
        } else {
            (*head).rbh_root = elm;
        }
        (*(*old).entry.rbe_left).entry.rbe_parent = elm;
        if !((*old).entry.rbe_right).is_null() {
            (*(*old).entry.rbe_right).entry.rbe_parent = elm;
        }
        if !parent.is_null() {
            left = parent;
            loop {
                left = (*left).entry.rbe_parent;
                if left.is_null() {
                    break;
                }
            }
        }
        current_block = 18344544714584178677;
    }
    match current_block {
        7245201122033322888 => {
            parent = (*elm).entry.rbe_parent;
            color = (*elm).entry.rbe_color;
            if !child.is_null() {
                (*child).entry.rbe_parent = parent;
            }
            if !parent.is_null() {
                if (*parent).entry.rbe_left == elm {
                    (*parent).entry.rbe_left = child;
                } else {
                    (*parent).entry.rbe_right = child;
                }
            } else {
                (*head).rbh_root = child;
            }
        }
        _ => {}
    }
    if color == 0 as libc::c_int {
        args_tree_RB_REMOVE_COLOR(head, parent, child);
    }
    return old;
}
unsafe extern "C" fn args_tree_RB_REMOVE_COLOR(
    mut head: *mut args_tree,
    mut parent: *mut args_entry,
    mut elm: *mut args_entry,
) {
    let mut tmp: *mut args_entry = 0 as *mut args_entry;
    while (elm.is_null() || (*elm).entry.rbe_color == 0 as libc::c_int)
        && elm != (*head).rbh_root
    {
        if (*parent).entry.rbe_left == elm {
            tmp = (*parent).entry.rbe_right;
            if (*tmp).entry.rbe_color == 1 as libc::c_int {
                (*tmp).entry.rbe_color = 0 as libc::c_int;
                (*parent).entry.rbe_color = 1 as libc::c_int;
                tmp = (*parent).entry.rbe_right;
                (*parent).entry.rbe_right = (*tmp).entry.rbe_left;
                if !((*parent).entry.rbe_right).is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_parent = parent;
                }
                (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                if !((*tmp).entry.rbe_parent).is_null() {
                    if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                        (*(*parent).entry.rbe_parent).entry.rbe_left = tmp;
                    } else {
                        (*(*parent).entry.rbe_parent).entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).entry.rbe_left = parent;
                (*parent).entry.rbe_parent = tmp;
                !((*tmp).entry.rbe_parent).is_null();
                tmp = (*parent).entry.rbe_right;
            }
            if (((*tmp).entry.rbe_left).is_null()
                || (*(*tmp).entry.rbe_left).entry.rbe_color == 0 as libc::c_int)
                && (((*tmp).entry.rbe_right).is_null()
                    || (*(*tmp).entry.rbe_right).entry.rbe_color == 0 as libc::c_int)
            {
                (*tmp).entry.rbe_color = 1 as libc::c_int;
                elm = parent;
                parent = (*elm).entry.rbe_parent;
            } else {
                if ((*tmp).entry.rbe_right).is_null()
                    || (*(*tmp).entry.rbe_right).entry.rbe_color == 0 as libc::c_int
                {
                    let mut oleft: *mut args_entry = 0 as *mut args_entry;
                    oleft = (*tmp).entry.rbe_left;
                    if !oleft.is_null() {
                        (*oleft).entry.rbe_color = 0 as libc::c_int;
                    }
                    (*tmp).entry.rbe_color = 1 as libc::c_int;
                    oleft = (*tmp).entry.rbe_left;
                    (*tmp).entry.rbe_left = (*oleft).entry.rbe_right;
                    if !((*tmp).entry.rbe_left).is_null() {
                        (*(*oleft).entry.rbe_right).entry.rbe_parent = tmp;
                    }
                    (*oleft).entry.rbe_parent = (*tmp).entry.rbe_parent;
                    if !((*oleft).entry.rbe_parent).is_null() {
                        if tmp == (*(*tmp).entry.rbe_parent).entry.rbe_left {
                            (*(*tmp).entry.rbe_parent).entry.rbe_left = oleft;
                        } else {
                            (*(*tmp).entry.rbe_parent).entry.rbe_right = oleft;
                        }
                    } else {
                        (*head).rbh_root = oleft;
                    }
                    (*oleft).entry.rbe_right = tmp;
                    (*tmp).entry.rbe_parent = oleft;
                    !((*oleft).entry.rbe_parent).is_null();
                    tmp = (*parent).entry.rbe_right;
                }
                (*tmp).entry.rbe_color = (*parent).entry.rbe_color;
                (*parent).entry.rbe_color = 0 as libc::c_int;
                if !((*tmp).entry.rbe_right).is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_color = 0 as libc::c_int;
                }
                tmp = (*parent).entry.rbe_right;
                (*parent).entry.rbe_right = (*tmp).entry.rbe_left;
                if !((*parent).entry.rbe_right).is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_parent = parent;
                }
                (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                if !((*tmp).entry.rbe_parent).is_null() {
                    if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                        (*(*parent).entry.rbe_parent).entry.rbe_left = tmp;
                    } else {
                        (*(*parent).entry.rbe_parent).entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).entry.rbe_left = parent;
                (*parent).entry.rbe_parent = tmp;
                !((*tmp).entry.rbe_parent).is_null();
                elm = (*head).rbh_root;
                break;
            }
        } else {
            tmp = (*parent).entry.rbe_left;
            if (*tmp).entry.rbe_color == 1 as libc::c_int {
                (*tmp).entry.rbe_color = 0 as libc::c_int;
                (*parent).entry.rbe_color = 1 as libc::c_int;
                tmp = (*parent).entry.rbe_left;
                (*parent).entry.rbe_left = (*tmp).entry.rbe_right;
                if !((*parent).entry.rbe_left).is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_parent = parent;
                }
                (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                if !((*tmp).entry.rbe_parent).is_null() {
                    if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                        (*(*parent).entry.rbe_parent).entry.rbe_left = tmp;
                    } else {
                        (*(*parent).entry.rbe_parent).entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).entry.rbe_right = parent;
                (*parent).entry.rbe_parent = tmp;
                !((*tmp).entry.rbe_parent).is_null();
                tmp = (*parent).entry.rbe_left;
            }
            if (((*tmp).entry.rbe_left).is_null()
                || (*(*tmp).entry.rbe_left).entry.rbe_color == 0 as libc::c_int)
                && (((*tmp).entry.rbe_right).is_null()
                    || (*(*tmp).entry.rbe_right).entry.rbe_color == 0 as libc::c_int)
            {
                (*tmp).entry.rbe_color = 1 as libc::c_int;
                elm = parent;
                parent = (*elm).entry.rbe_parent;
            } else {
                if ((*tmp).entry.rbe_left).is_null()
                    || (*(*tmp).entry.rbe_left).entry.rbe_color == 0 as libc::c_int
                {
                    let mut oright: *mut args_entry = 0 as *mut args_entry;
                    oright = (*tmp).entry.rbe_right;
                    if !oright.is_null() {
                        (*oright).entry.rbe_color = 0 as libc::c_int;
                    }
                    (*tmp).entry.rbe_color = 1 as libc::c_int;
                    oright = (*tmp).entry.rbe_right;
                    (*tmp).entry.rbe_right = (*oright).entry.rbe_left;
                    if !((*tmp).entry.rbe_right).is_null() {
                        (*(*oright).entry.rbe_left).entry.rbe_parent = tmp;
                    }
                    (*oright).entry.rbe_parent = (*tmp).entry.rbe_parent;
                    if !((*oright).entry.rbe_parent).is_null() {
                        if tmp == (*(*tmp).entry.rbe_parent).entry.rbe_left {
                            (*(*tmp).entry.rbe_parent).entry.rbe_left = oright;
                        } else {
                            (*(*tmp).entry.rbe_parent).entry.rbe_right = oright;
                        }
                    } else {
                        (*head).rbh_root = oright;
                    }
                    (*oright).entry.rbe_left = tmp;
                    (*tmp).entry.rbe_parent = oright;
                    !((*oright).entry.rbe_parent).is_null();
                    tmp = (*parent).entry.rbe_left;
                }
                (*tmp).entry.rbe_color = (*parent).entry.rbe_color;
                (*parent).entry.rbe_color = 0 as libc::c_int;
                if !((*tmp).entry.rbe_left).is_null() {
                    (*(*tmp).entry.rbe_left).entry.rbe_color = 0 as libc::c_int;
                }
                tmp = (*parent).entry.rbe_left;
                (*parent).entry.rbe_left = (*tmp).entry.rbe_right;
                if !((*parent).entry.rbe_left).is_null() {
                    (*(*tmp).entry.rbe_right).entry.rbe_parent = parent;
                }
                (*tmp).entry.rbe_parent = (*parent).entry.rbe_parent;
                if !((*tmp).entry.rbe_parent).is_null() {
                    if parent == (*(*parent).entry.rbe_parent).entry.rbe_left {
                        (*(*parent).entry.rbe_parent).entry.rbe_left = tmp;
                    } else {
                        (*(*parent).entry.rbe_parent).entry.rbe_right = tmp;
                    }
                } else {
                    (*head).rbh_root = tmp;
                }
                (*tmp).entry.rbe_right = parent;
                (*parent).entry.rbe_parent = tmp;
                !((*tmp).entry.rbe_parent).is_null();
                elm = (*head).rbh_root;
                break;
            }
        }
    }
    if !elm.is_null() {
        (*elm).entry.rbe_color = 0 as libc::c_int;
    }
}
unsafe extern "C" fn args_tree_RB_NEXT(mut elm: *mut args_entry) -> *mut args_entry {
    if !((*elm).entry.rbe_right).is_null() {
        elm = (*elm).entry.rbe_right;
        while !((*elm).entry.rbe_left).is_null() {
            elm = (*elm).entry.rbe_left;
        }
    } else if !((*elm).entry.rbe_parent).is_null()
        && elm == (*(*elm).entry.rbe_parent).entry.rbe_left
    {
        elm = (*elm).entry.rbe_parent;
    } else {
        while !((*elm).entry.rbe_parent).is_null()
            && elm == (*(*elm).entry.rbe_parent).entry.rbe_right
        {
            elm = (*elm).entry.rbe_parent;
        }
        elm = (*elm).entry.rbe_parent;
    }
    return elm;
}
unsafe extern "C" fn args_tree_RB_INSERT(
    mut head: *mut args_tree,
    mut elm: *mut args_entry,
) -> *mut args_entry {
    let mut tmp: *mut args_entry = 0 as *mut args_entry;
    let mut parent: *mut args_entry = 0 as *mut args_entry;
    let mut comp: libc::c_int = 0 as libc::c_int;
    tmp = (*head).rbh_root;
    while !tmp.is_null() {
        parent = tmp;
        comp = args_cmp(elm, parent);
        if comp < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left;
        } else if comp > 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_right;
        } else {
            return tmp
        }
    }
    (*elm).entry.rbe_parent = parent;
    (*elm).entry.rbe_right = 0 as *mut args_entry;
    (*elm).entry.rbe_left = (*elm).entry.rbe_right;
    (*elm).entry.rbe_color = 1 as libc::c_int;
    if !parent.is_null() {
        if comp < 0 as libc::c_int {
            (*parent).entry.rbe_left = elm;
        } else {
            (*parent).entry.rbe_right = elm;
        }
    } else {
        (*head).rbh_root = elm;
    }
    args_tree_RB_INSERT_COLOR(head, elm);
    return 0 as *mut args_entry;
}
unsafe extern "C" fn args_tree_RB_MINMAX(
    mut head: *mut args_tree,
    mut val: libc::c_int,
) -> *mut args_entry {
    let mut tmp: *mut args_entry = (*head).rbh_root;
    let mut parent: *mut args_entry = 0 as *mut args_entry;
    while !tmp.is_null() {
        parent = tmp;
        if val < 0 as libc::c_int {
            tmp = (*tmp).entry.rbe_left;
        } else {
            tmp = (*tmp).entry.rbe_right;
        }
    }
    return parent;
}
unsafe extern "C" fn args_cmp(
    mut a1: *mut args_entry,
    mut a2: *mut args_entry,
) -> libc::c_int {
    return (*a1).flag as libc::c_int - (*a2).flag as libc::c_int;
}
unsafe extern "C" fn args_find(mut args: *mut args, mut ch: u_char) -> *mut args_entry {
    let mut entry: args_entry = args_entry {
        flag: 0,
        value: 0 as *mut libc::c_char,
        entry: C2RustUnnamed {
            rbe_left: 0 as *mut args_entry,
            rbe_right: 0 as *mut args_entry,
            rbe_parent: 0 as *mut args_entry,
            rbe_color: 0,
        },
    };
    entry.flag = ch;
    return args_tree_RB_FIND(&mut (*args).tree, &mut entry);
}
#[no_mangle]
pub unsafe extern "C" fn args_parse(
    mut template: *const libc::c_char,
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> *mut args {
    let mut args: *mut args = 0 as *mut args;
    let mut opt: libc::c_int = 0;
    args = xcalloc(
        1 as libc::c_int as size_t,
        ::core::mem::size_of::<args>() as libc::c_ulong,
    ) as *mut args;
    BSDoptreset = 1 as libc::c_int;
    BSDoptind = 1 as libc::c_int;
    loop {
        opt = BSDgetopt(argc, argv, template);
        if !(opt != -(1 as libc::c_int)) {
            break;
        }
        if opt < 0 as libc::c_int {
            continue;
        }
        if opt == '?' as i32 || (strchr(template, opt)).is_null() {
            args_free(args);
            return 0 as *mut args;
        }
        args_set(args, opt as u_char, BSDoptarg);
    }
    argc -= BSDoptind;
    argv = argv.offset(BSDoptind as isize);
    (*args).argc = argc;
    (*args).argv = cmd_copy_argv(argc, argv);
    return args;
}
#[no_mangle]
pub unsafe extern "C" fn args_free(mut args: *mut args) {
    let mut entry: *mut args_entry = 0 as *mut args_entry;
    let mut entry1: *mut args_entry = 0 as *mut args_entry;
    cmd_free_argv((*args).argc, (*args).argv);
    entry = args_tree_RB_MINMAX(&mut (*args).tree, -(1 as libc::c_int));
    while !entry.is_null()
        && {
            entry1 = args_tree_RB_NEXT(entry);
            1 as libc::c_int != 0
        }
    {
        args_tree_RB_REMOVE(&mut (*args).tree, entry);
        free((*entry).value as *mut libc::c_void);
        free(entry as *mut libc::c_void);
        entry = entry1;
    }
    free(args as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn args_print(mut args: *mut args) -> *mut libc::c_char {
    let mut len: size_t = 0;
    let mut buf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut escaped: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut i: libc::c_int = 0;
    let mut flags: libc::c_int = 0;
    let mut entry: *mut args_entry = 0 as *mut args_entry;
    static mut quoted: [libc::c_char; 7] = unsafe {
        *::core::mem::transmute::<&[u8; 7], &[libc::c_char; 7]>(b" #\"';$\0")
    };
    len = 1 as libc::c_int as size_t;
    buf = xcalloc(1 as libc::c_int as size_t, len) as *mut libc::c_char;
    entry = args_tree_RB_MINMAX(&mut (*args).tree, -(1 as libc::c_int));
    while !entry.is_null() {
        if ((*entry).value).is_null() {
            if *buf as libc::c_int == '\0' as i32 {
                args_print_add(
                    &mut buf as *mut *mut libc::c_char,
                    &mut len as *mut size_t,
                    b"-\0" as *const u8 as *const libc::c_char,
                );
            }
            args_print_add(
                &mut buf as *mut *mut libc::c_char,
                &mut len as *mut size_t,
                b"%c\0" as *const u8 as *const libc::c_char,
                (*entry).flag as libc::c_int,
            );
        }
        entry = args_tree_RB_NEXT(entry);
    }
    entry = args_tree_RB_MINMAX(&mut (*args).tree, -(1 as libc::c_int));
    while !entry.is_null() {
        if !((*entry).value).is_null() {
            if *buf as libc::c_int != '\0' as i32 {
                args_print_add(
                    &mut buf as *mut *mut libc::c_char,
                    &mut len as *mut size_t,
                    b" -%c \0" as *const u8 as *const libc::c_char,
                    (*entry).flag as libc::c_int,
                );
            } else {
                args_print_add(
                    &mut buf as *mut *mut libc::c_char,
                    &mut len as *mut size_t,
                    b"-%c \0" as *const u8 as *const libc::c_char,
                    (*entry).flag as libc::c_int,
                );
            }
            flags = 0x1 as libc::c_int | 0x8 as libc::c_int | 0x10 as libc::c_int;
            if *((*entry).value)
                .offset(strcspn((*entry).value, quoted.as_ptr()) as isize) as libc::c_int
                != '\0' as i32
            {
                flags |= 0x200 as libc::c_int;
            }
            utf8_stravis(&mut escaped, (*entry).value, flags);
            if flags & 0x200 as libc::c_int != 0 {
                args_print_add(
                    &mut buf as *mut *mut libc::c_char,
                    &mut len as *mut size_t,
                    b"\"%s\"\0" as *const u8 as *const libc::c_char,
                    escaped,
                );
            } else {
                args_print_add(
                    &mut buf as *mut *mut libc::c_char,
                    &mut len as *mut size_t,
                    b"%s\0" as *const u8 as *const libc::c_char,
                    escaped,
                );
            }
            free(escaped as *mut libc::c_void);
        }
        entry = args_tree_RB_NEXT(entry);
    }
    i = 0 as libc::c_int;
    while i < (*args).argc {
        if *buf as libc::c_int != '\0' as i32 {
            args_print_add(
                &mut buf as *mut *mut libc::c_char,
                &mut len as *mut size_t,
                b" \0" as *const u8 as *const libc::c_char,
            );
        }
        flags = 0x1 as libc::c_int | 0x8 as libc::c_int | 0x10 as libc::c_int;
        if *(*((*args).argv).offset(i as isize))
            .offset(
                strcspn(*((*args).argv).offset(i as isize), quoted.as_ptr()) as isize,
            ) as libc::c_int != '\0' as i32
        {
            flags |= 0x200 as libc::c_int;
        }
        utf8_stravis(&mut escaped, *((*args).argv).offset(i as isize), flags);
        if flags & 0x200 as libc::c_int != 0 {
            args_print_add(
                &mut buf as *mut *mut libc::c_char,
                &mut len as *mut size_t,
                b"\"%s\"\0" as *const u8 as *const libc::c_char,
                escaped,
            );
        } else {
            args_print_add(
                &mut buf as *mut *mut libc::c_char,
                &mut len as *mut size_t,
                b"%s\0" as *const u8 as *const libc::c_char,
                escaped,
            );
        }
        free(escaped as *mut libc::c_void);
        i += 1;
        i;
    }
    return buf;
}
#[no_mangle]
pub unsafe extern "C" fn args_has(mut args: *mut args, mut ch: u_char) -> libc::c_int {
    return (args_find(args, ch) != 0 as *mut libc::c_void as *mut args_entry)
        as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn args_set(
    mut args: *mut args,
    mut ch: u_char,
    mut value: *const libc::c_char,
) {
    let mut entry: *mut args_entry = 0 as *mut args_entry;
    entry = args_find(args, ch);
    if !entry.is_null() {
        free((*entry).value as *mut libc::c_void);
        (*entry).value = 0 as *mut libc::c_char;
    } else {
        entry = xcalloc(
            1 as libc::c_int as size_t,
            ::core::mem::size_of::<args_entry>() as libc::c_ulong,
        ) as *mut args_entry;
        (*entry).flag = ch;
        args_tree_RB_INSERT(&mut (*args).tree, entry);
    }
    if !value.is_null() {
        (*entry).value = xstrdup(value);
    }
}
#[no_mangle]
pub unsafe extern "C" fn args_get(
    mut args: *mut args,
    mut ch: u_char,
) -> *const libc::c_char {
    let mut entry: *mut args_entry = 0 as *mut args_entry;
    entry = args_find(args, ch);
    if entry.is_null() {
        return 0 as *const libc::c_char;
    }
    return (*entry).value;
}
#[no_mangle]
pub unsafe extern "C" fn args_strtonum(
    mut args: *mut args,
    mut ch: u_char,
    mut minval: libc::c_longlong,
    mut maxval: libc::c_longlong,
    mut cause: *mut *mut libc::c_char,
) -> libc::c_longlong {
    let mut errstr: *const libc::c_char = 0 as *const libc::c_char;
    let mut ll: libc::c_longlong = 0;
    let mut entry: *mut args_entry = 0 as *mut args_entry;
    entry = args_find(args, ch);
    if entry.is_null() {
        *cause = xstrdup(b"missing\0" as *const u8 as *const libc::c_char);
        return 0 as libc::c_int as libc::c_longlong;
    }
    ll = strtonum((*entry).value, minval, maxval, &mut errstr);
    if !errstr.is_null() {
        *cause = xstrdup(errstr);
        return 0 as libc::c_int as libc::c_longlong;
    }
    *cause = 0 as *mut libc::c_char;
    return ll;
}
