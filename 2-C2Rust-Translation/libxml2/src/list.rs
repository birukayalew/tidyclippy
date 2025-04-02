use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    static mut xmlMalloc: xmlMallocFunc;
    static mut xmlFree: xmlFreeFunc;
    fn __xmlGenericError() -> *mut xmlGenericErrorFunc;
    fn __xmlGenericErrorContext() -> *mut *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type xmlFreeFunc = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type xmlMallocFunc = Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>;
pub type xmlGenericErrorFunc = Option::<
    unsafe extern "C" fn(*mut libc::c_void, *const libc::c_char, ...) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlLink {
    pub next: *mut _xmlLink,
    pub prev: *mut _xmlLink,
    pub data: *mut libc::c_void,
}
pub type xmlLink = _xmlLink;
pub type xmlLinkPtr = *mut xmlLink;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlList {
    pub sentinel: xmlLinkPtr,
    pub linkDeallocator: Option::<unsafe extern "C" fn(xmlLinkPtr) -> ()>,
    pub linkCompare: Option::<
        unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
    >,
}
pub type xmlList = _xmlList;
pub type xmlListPtr = *mut xmlList;
pub type xmlListDeallocator = Option::<unsafe extern "C" fn(xmlLinkPtr) -> ()>;
pub type xmlListDataCompare = Option::<
    unsafe extern "C" fn(*const libc::c_void, *const libc::c_void) -> libc::c_int,
>;
pub type xmlListWalker = Option::<
    unsafe extern "C" fn(*const libc::c_void, *mut libc::c_void) -> libc::c_int,
>;
unsafe extern "C" fn xmlLinkDeallocator(mut l: xmlListPtr, mut lk: xmlLinkPtr) {
    (*(*lk).prev).next = (*lk).next;
    (*(*lk).next).prev = (*lk).prev;
    if ((*l).linkDeallocator).is_some() {
        ((*l).linkDeallocator).expect("non-null function pointer")(lk);
    }
    xmlFree.expect("non-null function pointer")(lk as *mut libc::c_void);
}
unsafe extern "C" fn xmlLinkCompare(
    mut data0: *const libc::c_void,
    mut data1: *const libc::c_void,
) -> libc::c_int {
    if data0 < data1 {
        return -(1 as libc::c_int)
    } else if data0 == data1 {
        return 0 as libc::c_int
    }
    return 1 as libc::c_int;
}
unsafe extern "C" fn xmlListLowerSearch(
    mut l: xmlListPtr,
    mut data: *mut libc::c_void,
) -> xmlLinkPtr {
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    if l.is_null() {
        return 0 as xmlLinkPtr;
    }
    lk = (*(*l).sentinel).next;
    while lk != (*l).sentinel
        && ((*l).linkCompare).expect("non-null function pointer")((*lk).data, data)
            < 0 as libc::c_int
    {
        lk = (*lk).next;
    }
    return lk;
}
unsafe extern "C" fn xmlListHigherSearch(
    mut l: xmlListPtr,
    mut data: *mut libc::c_void,
) -> xmlLinkPtr {
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    if l.is_null() {
        return 0 as xmlLinkPtr;
    }
    lk = (*(*l).sentinel).prev;
    while lk != (*l).sentinel
        && ((*l).linkCompare).expect("non-null function pointer")((*lk).data, data)
            > 0 as libc::c_int
    {
        lk = (*lk).prev;
    }
    return lk;
}
unsafe extern "C" fn xmlListLinkSearch(
    mut l: xmlListPtr,
    mut data: *mut libc::c_void,
) -> xmlLinkPtr {
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    if l.is_null() {
        return 0 as xmlLinkPtr;
    }
    lk = xmlListLowerSearch(l, data);
    if lk == (*l).sentinel {
        return 0 as xmlLinkPtr
    } else {
        if ((*l).linkCompare).expect("non-null function pointer")((*lk).data, data)
            == 0 as libc::c_int
        {
            return lk;
        }
        return 0 as xmlLinkPtr;
    };
}
unsafe extern "C" fn xmlListLinkReverseSearch(
    mut l: xmlListPtr,
    mut data: *mut libc::c_void,
) -> xmlLinkPtr {
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    if l.is_null() {
        return 0 as xmlLinkPtr;
    }
    lk = xmlListHigherSearch(l, data);
    if lk == (*l).sentinel {
        return 0 as xmlLinkPtr
    } else {
        if ((*l).linkCompare).expect("non-null function pointer")((*lk).data, data)
            == 0 as libc::c_int
        {
            return lk;
        }
        return 0 as xmlLinkPtr;
    };
}
#[no_mangle]
pub unsafe extern "C" fn xmlListCreate(
    mut deallocator: xmlListDeallocator,
    mut compare: xmlListDataCompare,
) -> xmlListPtr {
    let mut l: xmlListPtr = 0 as *mut xmlList;
    l = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlList>() as libc::c_ulong) as xmlListPtr;
    if l.is_null() {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Cannot initialize memory for list\0" as *const u8 as *const libc::c_char,
        );
        return 0 as xmlListPtr;
    }
    memset(
        l as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<xmlList>() as libc::c_ulong,
    );
    (*l)
        .sentinel = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlLink>() as libc::c_ulong) as xmlLinkPtr;
    if ((*l).sentinel).is_null() {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Cannot initialize memory for sentinel\0" as *const u8
                as *const libc::c_char,
        );
        xmlFree.expect("non-null function pointer")(l as *mut libc::c_void);
        return 0 as xmlListPtr;
    }
    (*(*l).sentinel).next = (*l).sentinel;
    (*(*l).sentinel).prev = (*l).sentinel;
    (*(*l).sentinel).data = 0 as *mut libc::c_void;
    if deallocator.is_some() {
        (*l).linkDeallocator = deallocator;
    }
    if compare.is_some() {
        (*l).linkCompare = compare;
    } else {
        (*l)
            .linkCompare = Some(
            xmlLinkCompare
                as unsafe extern "C" fn(
                    *const libc::c_void,
                    *const libc::c_void,
                ) -> libc::c_int,
        );
    }
    return l;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListSearch(
    mut l: xmlListPtr,
    mut data: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    if l.is_null() {
        return 0 as *mut libc::c_void;
    }
    lk = xmlListLinkSearch(l, data);
    if !lk.is_null() {
        return (*lk).data;
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListReverseSearch(
    mut l: xmlListPtr,
    mut data: *mut libc::c_void,
) -> *mut libc::c_void {
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    if l.is_null() {
        return 0 as *mut libc::c_void;
    }
    lk = xmlListLinkReverseSearch(l, data);
    if !lk.is_null() {
        return (*lk).data;
    }
    return 0 as *mut libc::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListInsert(
    mut l: xmlListPtr,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut lkPlace: xmlLinkPtr = 0 as *mut xmlLink;
    let mut lkNew: xmlLinkPtr = 0 as *mut xmlLink;
    if l.is_null() {
        return 1 as libc::c_int;
    }
    lkPlace = xmlListLowerSearch(l, data);
    lkNew = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlLink>() as libc::c_ulong) as xmlLinkPtr;
    if lkNew.is_null() {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Cannot initialize memory for new link\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    (*lkNew).data = data;
    lkPlace = (*lkPlace).prev;
    (*lkNew).next = (*lkPlace).next;
    (*(*lkPlace).next).prev = lkNew;
    (*lkPlace).next = lkNew;
    (*lkNew).prev = lkPlace;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListAppend(
    mut l: xmlListPtr,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut lkPlace: xmlLinkPtr = 0 as *mut xmlLink;
    let mut lkNew: xmlLinkPtr = 0 as *mut xmlLink;
    if l.is_null() {
        return 1 as libc::c_int;
    }
    lkPlace = xmlListHigherSearch(l, data);
    lkNew = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlLink>() as libc::c_ulong) as xmlLinkPtr;
    if lkNew.is_null() {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Cannot initialize memory for new link\0" as *const u8
                as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    (*lkNew).data = data;
    (*lkNew).next = (*lkPlace).next;
    (*(*lkPlace).next).prev = lkNew;
    (*lkPlace).next = lkNew;
    (*lkNew).prev = lkPlace;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListDelete(mut l: xmlListPtr) {
    if l.is_null() {
        return;
    }
    xmlListClear(l);
    xmlFree.expect("non-null function pointer")((*l).sentinel as *mut libc::c_void);
    xmlFree.expect("non-null function pointer")(l as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlListRemoveFirst(
    mut l: xmlListPtr,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    if l.is_null() {
        return 0 as libc::c_int;
    }
    lk = xmlListLinkSearch(l, data);
    if !lk.is_null() {
        xmlLinkDeallocator(l, lk);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListRemoveLast(
    mut l: xmlListPtr,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    if l.is_null() {
        return 0 as libc::c_int;
    }
    lk = xmlListLinkReverseSearch(l, data);
    if !lk.is_null() {
        xmlLinkDeallocator(l, lk);
        return 1 as libc::c_int;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListRemoveAll(
    mut l: xmlListPtr,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut count: libc::c_int = 0 as libc::c_int;
    if l.is_null() {
        return 0 as libc::c_int;
    }
    while xmlListRemoveFirst(l, data) != 0 {
        count += 1;
        count;
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListClear(mut l: xmlListPtr) {
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    if l.is_null() {
        return;
    }
    lk = (*(*l).sentinel).next;
    while lk != (*l).sentinel {
        let mut next: xmlLinkPtr = (*lk).next;
        xmlLinkDeallocator(l, lk);
        lk = next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlListEmpty(mut l: xmlListPtr) -> libc::c_int {
    if l.is_null() {
        return -(1 as libc::c_int);
    }
    return ((*(*l).sentinel).next == (*l).sentinel) as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListFront(mut l: xmlListPtr) -> xmlLinkPtr {
    if l.is_null() {
        return 0 as xmlLinkPtr;
    }
    return (*(*l).sentinel).next;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListEnd(mut l: xmlListPtr) -> xmlLinkPtr {
    if l.is_null() {
        return 0 as xmlLinkPtr;
    }
    return (*(*l).sentinel).prev;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListSize(mut l: xmlListPtr) -> libc::c_int {
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut count: libc::c_int = 0 as libc::c_int;
    if l.is_null() {
        return -(1 as libc::c_int);
    }
    lk = (*(*l).sentinel).next;
    while lk != (*l).sentinel {
        lk = (*lk).next;
        count += 1;
        count;
    }
    return count;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListPopFront(mut l: xmlListPtr) {
    if xmlListEmpty(l) == 0 {
        xmlLinkDeallocator(l, (*(*l).sentinel).next);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlListPopBack(mut l: xmlListPtr) {
    if xmlListEmpty(l) == 0 {
        xmlLinkDeallocator(l, (*(*l).sentinel).prev);
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlListPushFront(
    mut l: xmlListPtr,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut lkPlace: xmlLinkPtr = 0 as *mut xmlLink;
    let mut lkNew: xmlLinkPtr = 0 as *mut xmlLink;
    if l.is_null() {
        return 0 as libc::c_int;
    }
    lkPlace = (*l).sentinel;
    lkNew = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlLink>() as libc::c_ulong) as xmlLinkPtr;
    if lkNew.is_null() {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Cannot initialize memory for new link\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    (*lkNew).data = data;
    (*lkNew).next = (*lkPlace).next;
    (*(*lkPlace).next).prev = lkNew;
    (*lkPlace).next = lkNew;
    (*lkNew).prev = lkPlace;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListPushBack(
    mut l: xmlListPtr,
    mut data: *mut libc::c_void,
) -> libc::c_int {
    let mut lkPlace: xmlLinkPtr = 0 as *mut xmlLink;
    let mut lkNew: xmlLinkPtr = 0 as *mut xmlLink;
    if l.is_null() {
        return 0 as libc::c_int;
    }
    lkPlace = (*(*l).sentinel).prev;
    lkNew = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlLink>() as libc::c_ulong) as xmlLinkPtr;
    if lkNew.is_null() {
        (*__xmlGenericError())
            .expect(
                "non-null function pointer",
            )(
            *__xmlGenericErrorContext(),
            b"Cannot initialize memory for new link\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    (*lkNew).data = data;
    (*lkNew).next = (*lkPlace).next;
    (*(*lkPlace).next).prev = lkNew;
    (*lkPlace).next = lkNew;
    (*lkNew).prev = lkPlace;
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlLinkGetData(mut lk: xmlLinkPtr) -> *mut libc::c_void {
    if lk.is_null() {
        return 0 as *mut libc::c_void;
    }
    return (*lk).data;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListReverse(mut l: xmlListPtr) {
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    let mut lkPrev: xmlLinkPtr = 0 as *mut xmlLink;
    if l.is_null() {
        return;
    }
    lkPrev = (*l).sentinel;
    lk = (*(*l).sentinel).next;
    while lk != (*l).sentinel {
        (*lkPrev).next = (*lkPrev).prev;
        (*lkPrev).prev = lk;
        lkPrev = lk;
        lk = (*lk).next;
    }
    (*lkPrev).next = (*lkPrev).prev;
    (*lkPrev).prev = lk;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListSort(mut l: xmlListPtr) {
    let mut lTemp: xmlListPtr = 0 as *mut xmlList;
    if l.is_null() {
        return;
    }
    if xmlListEmpty(l) != 0 {
        return;
    }
    lTemp = xmlListDup(l);
    if lTemp.is_null() {
        return;
    }
    xmlListClear(l);
    xmlListMerge(l, lTemp);
    xmlListDelete(lTemp);
}
#[no_mangle]
pub unsafe extern "C" fn xmlListWalk(
    mut l: xmlListPtr,
    mut walker: xmlListWalker,
    mut user: *mut libc::c_void,
) {
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    if l.is_null() || walker.is_none() {
        return;
    }
    lk = (*(*l).sentinel).next;
    while lk != (*l).sentinel {
        if walker.expect("non-null function pointer")((*lk).data, user)
            == 0 as libc::c_int
        {
            break;
        }
        lk = (*lk).next;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlListReverseWalk(
    mut l: xmlListPtr,
    mut walker: xmlListWalker,
    mut user: *mut libc::c_void,
) {
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    if l.is_null() || walker.is_none() {
        return;
    }
    lk = (*(*l).sentinel).prev;
    while lk != (*l).sentinel {
        if walker.expect("non-null function pointer")((*lk).data, user)
            == 0 as libc::c_int
        {
            break;
        }
        lk = (*lk).prev;
    }
}
#[no_mangle]
pub unsafe extern "C" fn xmlListMerge(mut l1: xmlListPtr, mut l2: xmlListPtr) {
    xmlListCopy(l1, l2);
    xmlListClear(l2);
}
#[no_mangle]
pub unsafe extern "C" fn xmlListDup(old: xmlListPtr) -> xmlListPtr {
    let mut cur: xmlListPtr = 0 as *mut xmlList;
    if old.is_null() {
        return 0 as xmlListPtr;
    }
    cur = xmlListCreate(None, (*old).linkCompare);
    if cur.is_null() {
        return 0 as xmlListPtr;
    }
    if 0 as libc::c_int != xmlListCopy(cur, old) {
        return 0 as xmlListPtr;
    }
    return cur;
}
#[no_mangle]
pub unsafe extern "C" fn xmlListCopy(
    mut cur: xmlListPtr,
    old: xmlListPtr,
) -> libc::c_int {
    let mut lk: xmlLinkPtr = 0 as *mut xmlLink;
    if old.is_null() || cur.is_null() {
        return 1 as libc::c_int;
    }
    lk = (*(*old).sentinel).next;
    while lk != (*old).sentinel {
        if 0 as libc::c_int != xmlListInsert(cur, (*lk).data) {
            xmlListDelete(cur);
            return 1 as libc::c_int;
        }
        lk = (*lk).next;
    }
    return 0 as libc::c_int;
}
