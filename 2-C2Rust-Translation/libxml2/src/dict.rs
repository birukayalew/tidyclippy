use ::libc;
extern "C" {
    pub type _xmlRMutex;
    fn xmlStrQEqual(
        pref: *const xmlChar,
        name: *const xmlChar,
        str: *const xmlChar,
    ) -> libc::c_int;
    fn rand_r(__seed: *mut libc::c_uint) -> libc::c_int;
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
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn xmlNewRMutex() -> xmlRMutexPtr;
    fn xmlRMutexLock(tok: xmlRMutexPtr);
    fn xmlRMutexUnlock(tok: xmlRMutexPtr);
    fn xmlFreeRMutex(tok: xmlRMutexPtr);
    static mut xmlMalloc: xmlMallocFunc;
    static mut xmlFree: xmlFreeFunc;
}
pub type xmlChar = libc::c_uchar;
pub type size_t = libc::c_ulong;
pub type __uint32_t = libc::c_uint;
pub type xmlRMutexPtr = *mut xmlRMutex;
pub type xmlRMutex = _xmlRMutex;
pub type uint32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDict {
    pub ref_counter: libc::c_int,
    pub dict: *mut _xmlDictEntry,
    pub size: size_t,
    pub nbElems: libc::c_uint,
    pub strings: xmlDictStringsPtr,
    pub subdict: *mut _xmlDict,
    pub seed: libc::c_int,
    pub limit: size_t,
}
pub type xmlDictStringsPtr = *mut xmlDictStrings;
pub type xmlDictStrings = _xmlDictStrings;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDictStrings {
    pub next: xmlDictStringsPtr,
    pub free: *mut xmlChar,
    pub end: *mut xmlChar,
    pub size: size_t,
    pub nbStrings: size_t,
    pub array: [xmlChar; 0],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _xmlDictEntry {
    pub next: *mut _xmlDictEntry,
    pub name: *const xmlChar,
    pub len: libc::c_uint,
    pub valid: libc::c_int,
    pub okey: libc::c_ulong,
}
pub type xmlDictPtr = *mut xmlDict;
pub type xmlDict = _xmlDict;
pub type xmlFreeFunc = Option::<unsafe extern "C" fn(*mut libc::c_void) -> ()>;
pub type xmlDictEntry = _xmlDictEntry;
pub type xmlMallocFunc = Option::<unsafe extern "C" fn(size_t) -> *mut libc::c_void>;
pub type xmlDictEntryPtr = *mut xmlDictEntry;
static mut xmlDictMutex: xmlRMutexPtr = 0 as *const xmlRMutex as xmlRMutexPtr;
static mut xmlDictInitialized: libc::c_int = 0 as libc::c_int;
static mut rand_seed: libc::c_uint = 0 as libc::c_int as libc::c_uint;
#[no_mangle]
pub unsafe extern "C" fn xmlInitializeDict() -> libc::c_int {
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlInitializeDict() -> libc::c_int {
    if xmlDictInitialized != 0 {
        return 1 as libc::c_int;
    }
    xmlDictMutex = xmlNewRMutex();
    if xmlDictMutex.is_null() {
        return 0 as libc::c_int;
    }
    xmlRMutexLock(xmlDictMutex);
    rand_seed = 0x12345678 as libc::c_int as libc::c_uint;
    rand_r(&mut rand_seed);
    xmlDictInitialized = 1 as libc::c_int;
    xmlRMutexUnlock(xmlDictMutex);
    return 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn __xmlRandom() -> libc::c_int {
    let mut ret: libc::c_int = 0;
    if xmlDictInitialized == 0 as libc::c_int {
        __xmlInitializeDict();
    }
    xmlRMutexLock(xmlDictMutex);
    ret = rand_r(&mut rand_seed);
    xmlRMutexUnlock(xmlDictMutex);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictCleanup() {
    if xmlDictInitialized == 0 {
        return;
    }
    xmlFreeRMutex(xmlDictMutex);
    xmlDictInitialized = 0 as libc::c_int;
}
unsafe extern "C" fn xmlDictAddString(
    mut dict: xmlDictPtr,
    mut name: *const xmlChar,
    mut namelen: libc::c_uint,
) -> *const xmlChar {
    let mut current_block: u64;
    let mut pool: xmlDictStringsPtr = 0 as *mut xmlDictStrings;
    let mut ret: *const xmlChar = 0 as *const xmlChar;
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut limit: size_t = 0 as libc::c_int as size_t;
    pool = (*dict).strings;
    loop {
        if pool.is_null() {
            current_block = 7351195479953500246;
            break;
        }
        if ((*pool).end).offset_from((*pool).free) as libc::c_long as size_t
            > namelen as size_t
        {
            current_block = 2031321738574369066;
            break;
        }
        if (*pool).size > size {
            size = (*pool).size;
        }
        limit = limit.wrapping_add((*pool).size);
        pool = (*pool).next;
    }
    match current_block {
        7351195479953500246 => {
            if pool.is_null() {
                if (*dict).limit > 0 as libc::c_int as size_t && limit > (*dict).limit {
                    return 0 as *const xmlChar;
                }
                if size == 0 as libc::c_int as size_t {
                    size = 1000 as libc::c_int as size_t;
                } else {
                    size = size * 4 as libc::c_int as size_t;
                }
                if size
                    < (4 as libc::c_int as libc::c_uint).wrapping_mul(namelen) as size_t
                {
                    size = (4 as libc::c_int as libc::c_uint).wrapping_mul(namelen)
                        as size_t;
                }
                pool = xmlMalloc
                    .expect(
                        "non-null function pointer",
                    )(
                    (::core::mem::size_of::<xmlDictStrings>() as libc::c_ulong)
                        .wrapping_add(size),
                ) as xmlDictStringsPtr;
                if pool.is_null() {
                    return 0 as *const xmlChar;
                }
                (*pool).size = size;
                (*pool).nbStrings = 0 as libc::c_int as size_t;
                (*pool)
                    .free = &mut *((*pool).array)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as *mut xmlChar;
                (*pool)
                    .end = &mut *((*pool).array).as_mut_ptr().offset(size as isize)
                    as *mut xmlChar;
                (*pool).next = (*dict).strings;
                (*dict).strings = pool;
            }
        }
        _ => {}
    }
    ret = (*pool).free;
    memcpy(
        (*pool).free as *mut libc::c_void,
        name as *const libc::c_void,
        namelen as libc::c_ulong,
    );
    (*pool).free = ((*pool).free).offset(namelen as isize);
    let fresh0 = (*pool).free;
    (*pool).free = ((*pool).free).offset(1);
    *fresh0 = 0 as libc::c_int as xmlChar;
    (*pool).nbStrings = ((*pool).nbStrings).wrapping_add(1);
    (*pool).nbStrings;
    return ret;
}
unsafe extern "C" fn xmlDictAddQString(
    mut dict: xmlDictPtr,
    mut prefix: *const xmlChar,
    mut plen: libc::c_uint,
    mut name: *const xmlChar,
    mut namelen: libc::c_uint,
) -> *const xmlChar {
    let mut current_block: u64;
    let mut pool: xmlDictStringsPtr = 0 as *mut xmlDictStrings;
    let mut ret: *const xmlChar = 0 as *const xmlChar;
    let mut size: size_t = 0 as libc::c_int as size_t;
    let mut limit: size_t = 0 as libc::c_int as size_t;
    if prefix.is_null() {
        return xmlDictAddString(dict, name, namelen);
    }
    pool = (*dict).strings;
    loop {
        if pool.is_null() {
            current_block = 13513818773234778473;
            break;
        }
        if ((*pool).end).offset_from((*pool).free) as libc::c_long as size_t
            > namelen.wrapping_add(plen).wrapping_add(1 as libc::c_int as libc::c_uint)
                as size_t
        {
            current_block = 14056734262066912179;
            break;
        }
        if (*pool).size > size {
            size = (*pool).size;
        }
        limit = limit.wrapping_add((*pool).size);
        pool = (*pool).next;
    }
    match current_block {
        13513818773234778473 => {
            if pool.is_null() {
                if (*dict).limit > 0 as libc::c_int as size_t && limit > (*dict).limit {
                    return 0 as *const xmlChar;
                }
                if size == 0 as libc::c_int as size_t {
                    size = 1000 as libc::c_int as size_t;
                } else {
                    size = size * 4 as libc::c_int as size_t;
                }
                if size
                    < (4 as libc::c_int as libc::c_uint)
                        .wrapping_mul(
                            namelen
                                .wrapping_add(plen)
                                .wrapping_add(1 as libc::c_int as libc::c_uint),
                        ) as size_t
                {
                    size = (4 as libc::c_int as libc::c_uint)
                        .wrapping_mul(
                            namelen
                                .wrapping_add(plen)
                                .wrapping_add(1 as libc::c_int as libc::c_uint),
                        ) as size_t;
                }
                pool = xmlMalloc
                    .expect(
                        "non-null function pointer",
                    )(
                    (::core::mem::size_of::<xmlDictStrings>() as libc::c_ulong)
                        .wrapping_add(size),
                ) as xmlDictStringsPtr;
                if pool.is_null() {
                    return 0 as *const xmlChar;
                }
                (*pool).size = size;
                (*pool).nbStrings = 0 as libc::c_int as size_t;
                (*pool)
                    .free = &mut *((*pool).array)
                    .as_mut_ptr()
                    .offset(0 as libc::c_int as isize) as *mut xmlChar;
                (*pool)
                    .end = &mut *((*pool).array).as_mut_ptr().offset(size as isize)
                    as *mut xmlChar;
                (*pool).next = (*dict).strings;
                (*dict).strings = pool;
            }
        }
        _ => {}
    }
    ret = (*pool).free;
    memcpy(
        (*pool).free as *mut libc::c_void,
        prefix as *const libc::c_void,
        plen as libc::c_ulong,
    );
    (*pool).free = ((*pool).free).offset(plen as isize);
    let fresh1 = (*pool).free;
    (*pool).free = ((*pool).free).offset(1);
    *fresh1 = ':' as i32 as xmlChar;
    memcpy(
        (*pool).free as *mut libc::c_void,
        name as *const libc::c_void,
        namelen as libc::c_ulong,
    );
    (*pool).free = ((*pool).free).offset(namelen as isize);
    let fresh2 = (*pool).free;
    (*pool).free = ((*pool).free).offset(1);
    *fresh2 = 0 as libc::c_int as xmlChar;
    (*pool).nbStrings = ((*pool).nbStrings).wrapping_add(1);
    (*pool).nbStrings;
    return ret;
}
unsafe extern "C" fn xmlDictComputeBigKey(
    mut data: *const xmlChar,
    mut namelen: libc::c_int,
    mut seed: libc::c_int,
) -> uint32_t {
    let mut hash: uint32_t = 0;
    let mut i: libc::c_int = 0;
    if namelen <= 0 as libc::c_int || data.is_null() {
        return 0 as libc::c_int as uint32_t;
    }
    hash = seed as uint32_t;
    i = 0 as libc::c_int;
    while i < namelen {
        hash = hash.wrapping_add(*data.offset(i as isize) as uint32_t);
        hash = hash.wrapping_add(hash << 10 as libc::c_int);
        hash ^= hash >> 6 as libc::c_int;
        i += 1;
        i;
    }
    hash = hash.wrapping_add(hash << 3 as libc::c_int);
    hash ^= hash >> 11 as libc::c_int;
    hash = hash.wrapping_add(hash << 15 as libc::c_int);
    return hash;
}
unsafe extern "C" fn xmlDictComputeBigQKey(
    mut prefix: *const xmlChar,
    mut plen: libc::c_int,
    mut name: *const xmlChar,
    mut len: libc::c_int,
    mut seed: libc::c_int,
) -> libc::c_ulong {
    let mut hash: uint32_t = 0;
    let mut i: libc::c_int = 0;
    hash = seed as uint32_t;
    i = 0 as libc::c_int;
    while i < plen {
        hash = hash.wrapping_add(*prefix.offset(i as isize) as uint32_t);
        hash = hash.wrapping_add(hash << 10 as libc::c_int);
        hash ^= hash >> 6 as libc::c_int;
        i += 1;
        i;
    }
    hash = hash.wrapping_add(':' as i32 as uint32_t);
    hash = hash.wrapping_add(hash << 10 as libc::c_int);
    hash ^= hash >> 6 as libc::c_int;
    i = 0 as libc::c_int;
    while i < len {
        hash = hash.wrapping_add(*name.offset(i as isize) as uint32_t);
        hash = hash.wrapping_add(hash << 10 as libc::c_int);
        hash ^= hash >> 6 as libc::c_int;
        i += 1;
        i;
    }
    hash = hash.wrapping_add(hash << 3 as libc::c_int);
    hash ^= hash >> 11 as libc::c_int;
    hash = hash.wrapping_add(hash << 15 as libc::c_int);
    return hash as libc::c_ulong;
}
unsafe extern "C" fn xmlDictComputeFastKey(
    mut name: *const xmlChar,
    mut namelen: libc::c_int,
    mut seed: libc::c_int,
) -> libc::c_ulong {
    let mut value: libc::c_ulong = seed as libc::c_ulong;
    if name.is_null() {
        return 0 as libc::c_int as libc::c_ulong;
    }
    value = *name as libc::c_ulong;
    value <<= 5 as libc::c_int;
    if namelen > 10 as libc::c_int {
        value = value
            .wrapping_add(
                *name.offset((namelen - 1 as libc::c_int) as isize) as libc::c_ulong,
            );
        namelen = 10 as libc::c_int;
    }
    let mut current_block_16: u64;
    match namelen {
        10 => {
            value = value
                .wrapping_add(*name.offset(9 as libc::c_int as isize) as libc::c_ulong);
            current_block_16 = 17908814879015121377;
        }
        9 => {
            current_block_16 = 17908814879015121377;
        }
        8 => {
            current_block_16 = 18041545848130130018;
        }
        7 => {
            current_block_16 = 11489031855839515445;
        }
        6 => {
            current_block_16 = 4791084557613369489;
        }
        5 => {
            current_block_16 = 5406873316800603912;
        }
        4 => {
            current_block_16 = 16821042458485256832;
        }
        3 => {
            current_block_16 = 9800678731924539679;
        }
        2 => {
            current_block_16 = 3212989285975388259;
        }
        _ => {
            current_block_16 = 8831408221741692167;
        }
    }
    match current_block_16 {
        17908814879015121377 => {
            value = value
                .wrapping_add(*name.offset(8 as libc::c_int as isize) as libc::c_ulong);
            current_block_16 = 18041545848130130018;
        }
        _ => {}
    }
    match current_block_16 {
        18041545848130130018 => {
            value = value
                .wrapping_add(*name.offset(7 as libc::c_int as isize) as libc::c_ulong);
            current_block_16 = 11489031855839515445;
        }
        _ => {}
    }
    match current_block_16 {
        11489031855839515445 => {
            value = value
                .wrapping_add(*name.offset(6 as libc::c_int as isize) as libc::c_ulong);
            current_block_16 = 4791084557613369489;
        }
        _ => {}
    }
    match current_block_16 {
        4791084557613369489 => {
            value = value
                .wrapping_add(*name.offset(5 as libc::c_int as isize) as libc::c_ulong);
            current_block_16 = 5406873316800603912;
        }
        _ => {}
    }
    match current_block_16 {
        5406873316800603912 => {
            value = value
                .wrapping_add(*name.offset(4 as libc::c_int as isize) as libc::c_ulong);
            current_block_16 = 16821042458485256832;
        }
        _ => {}
    }
    match current_block_16 {
        16821042458485256832 => {
            value = value
                .wrapping_add(*name.offset(3 as libc::c_int as isize) as libc::c_ulong);
            current_block_16 = 9800678731924539679;
        }
        _ => {}
    }
    match current_block_16 {
        9800678731924539679 => {
            value = value
                .wrapping_add(*name.offset(2 as libc::c_int as isize) as libc::c_ulong);
            current_block_16 = 3212989285975388259;
        }
        _ => {}
    }
    match current_block_16 {
        3212989285975388259 => {
            value = value
                .wrapping_add(*name.offset(1 as libc::c_int as isize) as libc::c_ulong);
        }
        _ => {}
    }
    return value;
}
unsafe extern "C" fn xmlDictComputeFastQKey(
    mut prefix: *const xmlChar,
    mut plen: libc::c_int,
    mut name: *const xmlChar,
    mut len: libc::c_int,
    mut seed: libc::c_int,
) -> libc::c_ulong {
    let mut value: libc::c_ulong = seed as libc::c_ulong;
    if plen == 0 as libc::c_int {
        value = value
            .wrapping_add(
                (30 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(':' as i32 as libc::c_ulong),
            );
    } else {
        value = value
            .wrapping_add((30 as libc::c_int * *prefix as libc::c_int) as libc::c_ulong);
    }
    if len > 10 as libc::c_int {
        let mut offset: libc::c_int = len - (plen + 1 as libc::c_int + 1 as libc::c_int);
        if offset < 0 as libc::c_int {
            offset = len - (10 as libc::c_int + 1 as libc::c_int);
        }
        value = value.wrapping_add(*name.offset(offset as isize) as libc::c_ulong);
        len = 10 as libc::c_int;
        if plen > 10 as libc::c_int {
            plen = 10 as libc::c_int;
        }
    }
    let mut current_block_20: u64;
    match plen {
        10 => {
            value = value
                .wrapping_add(
                    *prefix.offset(9 as libc::c_int as isize) as libc::c_ulong,
                );
            current_block_20 = 1754312783919406564;
        }
        9 => {
            current_block_20 = 1754312783919406564;
        }
        8 => {
            current_block_20 = 10841693350810660124;
        }
        7 => {
            current_block_20 = 8087584078174868210;
        }
        6 => {
            current_block_20 = 961758752094881866;
        }
        5 => {
            current_block_20 = 14866680452771743319;
        }
        4 => {
            current_block_20 = 10956000615562811496;
        }
        3 => {
            current_block_20 = 2084950470139227270;
        }
        2 => {
            current_block_20 = 10007135551684069293;
        }
        1 => {
            current_block_20 = 12081857612379946233;
        }
        _ => {
            current_block_20 = 11584701595673473500;
        }
    }
    match current_block_20 {
        1754312783919406564 => {
            value = value
                .wrapping_add(
                    *prefix.offset(8 as libc::c_int as isize) as libc::c_ulong,
                );
            current_block_20 = 10841693350810660124;
        }
        _ => {}
    }
    match current_block_20 {
        10841693350810660124 => {
            value = value
                .wrapping_add(
                    *prefix.offset(7 as libc::c_int as isize) as libc::c_ulong,
                );
            current_block_20 = 8087584078174868210;
        }
        _ => {}
    }
    match current_block_20 {
        8087584078174868210 => {
            value = value
                .wrapping_add(
                    *prefix.offset(6 as libc::c_int as isize) as libc::c_ulong,
                );
            current_block_20 = 961758752094881866;
        }
        _ => {}
    }
    match current_block_20 {
        961758752094881866 => {
            value = value
                .wrapping_add(
                    *prefix.offset(5 as libc::c_int as isize) as libc::c_ulong,
                );
            current_block_20 = 14866680452771743319;
        }
        _ => {}
    }
    match current_block_20 {
        14866680452771743319 => {
            value = value
                .wrapping_add(
                    *prefix.offset(4 as libc::c_int as isize) as libc::c_ulong,
                );
            current_block_20 = 10956000615562811496;
        }
        _ => {}
    }
    match current_block_20 {
        10956000615562811496 => {
            value = value
                .wrapping_add(
                    *prefix.offset(3 as libc::c_int as isize) as libc::c_ulong,
                );
            current_block_20 = 2084950470139227270;
        }
        _ => {}
    }
    match current_block_20 {
        2084950470139227270 => {
            value = value
                .wrapping_add(
                    *prefix.offset(2 as libc::c_int as isize) as libc::c_ulong,
                );
            current_block_20 = 10007135551684069293;
        }
        _ => {}
    }
    match current_block_20 {
        10007135551684069293 => {
            value = value
                .wrapping_add(
                    *prefix.offset(1 as libc::c_int as isize) as libc::c_ulong,
                );
            current_block_20 = 12081857612379946233;
        }
        _ => {}
    }
    match current_block_20 {
        12081857612379946233 => {
            value = value
                .wrapping_add(
                    *prefix.offset(0 as libc::c_int as isize) as libc::c_ulong,
                );
        }
        _ => {}
    }
    len -= plen;
    if len > 0 as libc::c_int {
        value = value.wrapping_add(':' as i32 as libc::c_ulong);
        len -= 1;
        len;
    }
    let mut current_block_36: u64;
    match len {
        10 => {
            value = value
                .wrapping_add(*name.offset(9 as libc::c_int as isize) as libc::c_ulong);
            current_block_36 = 14247455491035445839;
        }
        9 => {
            current_block_36 = 14247455491035445839;
        }
        8 => {
            current_block_36 = 12238411934909626695;
        }
        7 => {
            current_block_36 = 7873764389453924156;
        }
        6 => {
            current_block_36 = 2532774858405249957;
        }
        5 => {
            current_block_36 = 15885459085044940717;
        }
        4 => {
            current_block_36 = 7146806494120673115;
        }
        3 => {
            current_block_36 = 778246091621005526;
        }
        2 => {
            current_block_36 = 1219063372262262174;
        }
        1 => {
            current_block_36 = 3463118035518185632;
        }
        _ => {
            current_block_36 = 2604890879466389055;
        }
    }
    match current_block_36 {
        14247455491035445839 => {
            value = value
                .wrapping_add(*name.offset(8 as libc::c_int as isize) as libc::c_ulong);
            current_block_36 = 12238411934909626695;
        }
        _ => {}
    }
    match current_block_36 {
        12238411934909626695 => {
            value = value
                .wrapping_add(*name.offset(7 as libc::c_int as isize) as libc::c_ulong);
            current_block_36 = 7873764389453924156;
        }
        _ => {}
    }
    match current_block_36 {
        7873764389453924156 => {
            value = value
                .wrapping_add(*name.offset(6 as libc::c_int as isize) as libc::c_ulong);
            current_block_36 = 2532774858405249957;
        }
        _ => {}
    }
    match current_block_36 {
        2532774858405249957 => {
            value = value
                .wrapping_add(*name.offset(5 as libc::c_int as isize) as libc::c_ulong);
            current_block_36 = 15885459085044940717;
        }
        _ => {}
    }
    match current_block_36 {
        15885459085044940717 => {
            value = value
                .wrapping_add(*name.offset(4 as libc::c_int as isize) as libc::c_ulong);
            current_block_36 = 7146806494120673115;
        }
        _ => {}
    }
    match current_block_36 {
        7146806494120673115 => {
            value = value
                .wrapping_add(*name.offset(3 as libc::c_int as isize) as libc::c_ulong);
            current_block_36 = 778246091621005526;
        }
        _ => {}
    }
    match current_block_36 {
        778246091621005526 => {
            value = value
                .wrapping_add(*name.offset(2 as libc::c_int as isize) as libc::c_ulong);
            current_block_36 = 1219063372262262174;
        }
        _ => {}
    }
    match current_block_36 {
        1219063372262262174 => {
            value = value
                .wrapping_add(*name.offset(1 as libc::c_int as isize) as libc::c_ulong);
            current_block_36 = 3463118035518185632;
        }
        _ => {}
    }
    match current_block_36 {
        3463118035518185632 => {
            value = value
                .wrapping_add(*name.offset(0 as libc::c_int as isize) as libc::c_ulong);
        }
        _ => {}
    }
    return value;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictCreate() -> xmlDictPtr {
    let mut dict: xmlDictPtr = 0 as *mut xmlDict;
    if xmlDictInitialized == 0 {
        if __xmlInitializeDict() == 0 {
            return 0 as xmlDictPtr;
        }
    }
    dict = xmlMalloc
        .expect(
            "non-null function pointer",
        )(::core::mem::size_of::<xmlDict>() as libc::c_ulong) as xmlDictPtr;
    if !dict.is_null() {
        (*dict).ref_counter = 1 as libc::c_int;
        (*dict).limit = 0 as libc::c_int as size_t;
        (*dict).size = 128 as libc::c_int as size_t;
        (*dict).nbElems = 0 as libc::c_int as libc::c_uint;
        (*dict)
            .dict = xmlMalloc
            .expect(
                "non-null function pointer",
            )(
            (128 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<xmlDictEntry>() as libc::c_ulong),
        ) as *mut _xmlDictEntry;
        (*dict).strings = 0 as xmlDictStringsPtr;
        (*dict).subdict = 0 as *mut _xmlDict;
        if !((*dict).dict).is_null() {
            memset(
                (*dict).dict as *mut libc::c_void,
                0 as libc::c_int,
                (128 as libc::c_int as libc::c_ulong)
                    .wrapping_mul(
                        ::core::mem::size_of::<xmlDictEntry>() as libc::c_ulong,
                    ),
            );
            (*dict).seed = __xmlRandom();
            return dict;
        }
        xmlFree.expect("non-null function pointer")(dict as *mut libc::c_void);
    }
    return 0 as xmlDictPtr;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictCreateSub(mut sub: xmlDictPtr) -> xmlDictPtr {
    let mut dict: xmlDictPtr = xmlDictCreate();
    if !dict.is_null() && !sub.is_null() {
        (*dict).seed = (*sub).seed;
        (*dict).subdict = sub;
        xmlDictReference((*dict).subdict);
    }
    return dict;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictReference(mut dict: xmlDictPtr) -> libc::c_int {
    if xmlDictInitialized == 0 {
        if __xmlInitializeDict() == 0 {
            return -(1 as libc::c_int);
        }
    }
    if dict.is_null() {
        return -(1 as libc::c_int);
    }
    xmlRMutexLock(xmlDictMutex);
    (*dict).ref_counter += 1;
    (*dict).ref_counter;
    xmlRMutexUnlock(xmlDictMutex);
    return 0 as libc::c_int;
}
unsafe extern "C" fn xmlDictGrow(mut dict: xmlDictPtr, mut size: size_t) -> libc::c_int {
    let mut key: libc::c_ulong = 0;
    let mut okey: libc::c_ulong = 0;
    let mut oldsize: size_t = 0;
    let mut i: size_t = 0;
    let mut iter: xmlDictEntryPtr = 0 as *mut xmlDictEntry;
    let mut next: xmlDictEntryPtr = 0 as *mut xmlDictEntry;
    let mut olddict: *mut _xmlDictEntry = 0 as *mut _xmlDictEntry;
    let mut ret: libc::c_int = 0 as libc::c_int;
    let mut keep_keys: libc::c_int = 1 as libc::c_int;
    if dict.is_null() {
        return -(1 as libc::c_int);
    }
    if size < 8 as libc::c_int as size_t {
        return -(1 as libc::c_int);
    }
    if size > (8 as libc::c_int * 2048 as libc::c_int) as size_t {
        return -(1 as libc::c_int);
    }
    oldsize = (*dict).size;
    olddict = (*dict).dict;
    if olddict.is_null() {
        return -(1 as libc::c_int);
    }
    if oldsize == 128 as libc::c_int as size_t {
        keep_keys = 0 as libc::c_int;
    }
    (*dict)
        .dict = xmlMalloc
        .expect(
            "non-null function pointer",
        )(size.wrapping_mul(::core::mem::size_of::<xmlDictEntry>() as libc::c_ulong))
        as *mut _xmlDictEntry;
    if ((*dict).dict).is_null() {
        (*dict).dict = olddict;
        return -(1 as libc::c_int);
    }
    memset(
        (*dict).dict as *mut libc::c_void,
        0 as libc::c_int,
        size.wrapping_mul(::core::mem::size_of::<xmlDictEntry>() as libc::c_ulong),
    );
    (*dict).size = size;
    i = 0 as libc::c_int as size_t;
    while i < oldsize {
        if !((*olddict.offset(i as isize)).valid == 0 as libc::c_int) {
            if keep_keys != 0 {
                okey = (*olddict.offset(i as isize)).okey;
            } else {
                okey = if (*dict).size == 128 as libc::c_int as size_t {
                    xmlDictComputeFastKey(
                        (*olddict.offset(i as isize)).name,
                        (*olddict.offset(i as isize)).len as libc::c_int,
                        (*dict).seed,
                    )
                } else {
                    xmlDictComputeBigKey(
                        (*olddict.offset(i as isize)).name,
                        (*olddict.offset(i as isize)).len as libc::c_int,
                        (*dict).seed,
                    ) as libc::c_ulong
                };
            }
            key = okey.wrapping_rem((*dict).size);
            if (*((*dict).dict).offset(key as isize)).valid == 0 as libc::c_int {
                memcpy(
                    &mut *((*dict).dict).offset(key as isize) as *mut _xmlDictEntry
                        as *mut libc::c_void,
                    &mut *olddict.offset(i as isize) as *mut _xmlDictEntry
                        as *const libc::c_void,
                    ::core::mem::size_of::<xmlDictEntry>() as libc::c_ulong,
                );
                let ref mut fresh3 = (*((*dict).dict).offset(key as isize)).next;
                *fresh3 = 0 as *mut _xmlDictEntry;
                (*((*dict).dict).offset(key as isize)).okey = okey;
            } else {
                let mut entry: xmlDictEntryPtr = 0 as *mut xmlDictEntry;
                entry = xmlMalloc
                    .expect(
                        "non-null function pointer",
                    )(::core::mem::size_of::<xmlDictEntry>() as libc::c_ulong)
                    as xmlDictEntryPtr;
                if !entry.is_null() {
                    (*entry).name = (*olddict.offset(i as isize)).name;
                    (*entry).len = (*olddict.offset(i as isize)).len;
                    (*entry).okey = okey;
                    (*entry).next = (*((*dict).dict).offset(key as isize)).next;
                    (*entry).valid = 1 as libc::c_int;
                    let ref mut fresh4 = (*((*dict).dict).offset(key as isize)).next;
                    *fresh4 = entry;
                } else {
                    ret = -(1 as libc::c_int);
                }
            }
        }
        i = i.wrapping_add(1);
        i;
    }
    i = 0 as libc::c_int as size_t;
    while i < oldsize {
        iter = (*olddict.offset(i as isize)).next;
        while !iter.is_null() {
            next = (*iter).next;
            if keep_keys != 0 {
                okey = (*iter).okey;
            } else {
                okey = if (*dict).size == 128 as libc::c_int as size_t {
                    xmlDictComputeFastKey(
                        (*iter).name,
                        (*iter).len as libc::c_int,
                        (*dict).seed,
                    )
                } else {
                    xmlDictComputeBigKey(
                        (*iter).name,
                        (*iter).len as libc::c_int,
                        (*dict).seed,
                    ) as libc::c_ulong
                };
            }
            key = okey.wrapping_rem((*dict).size);
            if (*((*dict).dict).offset(key as isize)).valid == 0 as libc::c_int {
                memcpy(
                    &mut *((*dict).dict).offset(key as isize) as *mut _xmlDictEntry
                        as *mut libc::c_void,
                    iter as *const libc::c_void,
                    ::core::mem::size_of::<xmlDictEntry>() as libc::c_ulong,
                );
                let ref mut fresh5 = (*((*dict).dict).offset(key as isize)).next;
                *fresh5 = 0 as *mut _xmlDictEntry;
                (*((*dict).dict).offset(key as isize)).valid = 1 as libc::c_int;
                (*((*dict).dict).offset(key as isize)).okey = okey;
                xmlFree.expect("non-null function pointer")(iter as *mut libc::c_void);
            } else {
                (*iter).next = (*((*dict).dict).offset(key as isize)).next;
                (*iter).okey = okey;
                let ref mut fresh6 = (*((*dict).dict).offset(key as isize)).next;
                *fresh6 = iter;
            }
            iter = next;
        }
        i = i.wrapping_add(1);
        i;
    }
    xmlFree.expect("non-null function pointer")(olddict as *mut libc::c_void);
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictFree(mut dict: xmlDictPtr) {
    let mut i: size_t = 0;
    let mut iter: xmlDictEntryPtr = 0 as *mut xmlDictEntry;
    let mut next: xmlDictEntryPtr = 0 as *mut xmlDictEntry;
    let mut inside_dict: libc::c_int = 0 as libc::c_int;
    let mut pool: xmlDictStringsPtr = 0 as *mut xmlDictStrings;
    let mut nextp: xmlDictStringsPtr = 0 as *mut xmlDictStrings;
    if dict.is_null() {
        return;
    }
    if xmlDictInitialized == 0 {
        if __xmlInitializeDict() == 0 {
            return;
        }
    }
    xmlRMutexLock(xmlDictMutex);
    (*dict).ref_counter -= 1;
    (*dict).ref_counter;
    if (*dict).ref_counter > 0 as libc::c_int {
        xmlRMutexUnlock(xmlDictMutex);
        return;
    }
    xmlRMutexUnlock(xmlDictMutex);
    if !((*dict).subdict).is_null() {
        xmlDictFree((*dict).subdict);
    }
    if !((*dict).dict).is_null() {
        i = 0 as libc::c_int as size_t;
        while i < (*dict).size && (*dict).nbElems > 0 as libc::c_int as libc::c_uint {
            iter = &mut *((*dict).dict).offset(i as isize) as *mut _xmlDictEntry;
            if !((*iter).valid == 0 as libc::c_int) {
                inside_dict = 1 as libc::c_int;
                while !iter.is_null() {
                    next = (*iter).next;
                    if inside_dict == 0 {
                        xmlFree
                            .expect(
                                "non-null function pointer",
                            )(iter as *mut libc::c_void);
                    }
                    (*dict).nbElems = ((*dict).nbElems).wrapping_sub(1);
                    (*dict).nbElems;
                    inside_dict = 0 as libc::c_int;
                    iter = next;
                }
            }
            i = i.wrapping_add(1);
            i;
        }
        xmlFree.expect("non-null function pointer")((*dict).dict as *mut libc::c_void);
    }
    pool = (*dict).strings;
    while !pool.is_null() {
        nextp = (*pool).next;
        xmlFree.expect("non-null function pointer")(pool as *mut libc::c_void);
        pool = nextp;
    }
    xmlFree.expect("non-null function pointer")(dict as *mut libc::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictLookup(
    mut dict: xmlDictPtr,
    mut name: *const xmlChar,
    mut len: libc::c_int,
) -> *const xmlChar {
    let mut key: libc::c_ulong = 0;
    let mut okey: libc::c_ulong = 0;
    let mut nbi: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut entry: xmlDictEntryPtr = 0 as *mut xmlDictEntry;
    let mut insert: xmlDictEntryPtr = 0 as *mut xmlDictEntry;
    let mut ret: *const xmlChar = 0 as *const xmlChar;
    let mut l: libc::c_uint = 0;
    if dict.is_null() || name.is_null() {
        return 0 as *const xmlChar;
    }
    if len < 0 as libc::c_int {
        l = strlen(name as *const libc::c_char) as libc::c_uint;
    } else {
        l = len as libc::c_uint;
    }
    if (*dict).limit > 0 as libc::c_int as size_t && l as size_t >= (*dict).limit
        || l > (2147483647 as libc::c_int / 2 as libc::c_int) as libc::c_uint
    {
        return 0 as *const xmlChar;
    }
    okey = if (*dict).size == 128 as libc::c_int as size_t {
        xmlDictComputeFastKey(name, l as libc::c_int, (*dict).seed)
    } else {
        xmlDictComputeBigKey(name, l as libc::c_int, (*dict).seed) as libc::c_ulong
    };
    key = okey.wrapping_rem((*dict).size);
    if (*((*dict).dict).offset(key as isize)).valid == 0 as libc::c_int {
        insert = 0 as xmlDictEntryPtr;
    } else {
        insert = &mut *((*dict).dict).offset(key as isize) as *mut _xmlDictEntry;
        while !((*insert).next).is_null() {
            if (*insert).okey == okey && (*insert).len == l {
                if memcmp(
                    (*insert).name as *const libc::c_void,
                    name as *const libc::c_void,
                    l as libc::c_ulong,
                ) == 0
                {
                    return (*insert).name;
                }
            }
            nbi = nbi.wrapping_add(1);
            nbi;
            insert = (*insert).next;
        }
        if (*insert).okey == okey && (*insert).len == l {
            if memcmp(
                (*insert).name as *const libc::c_void,
                name as *const libc::c_void,
                l as libc::c_ulong,
            ) == 0
            {
                return (*insert).name;
            }
        }
    }
    if !((*dict).subdict).is_null() {
        let mut skey: libc::c_ulong = 0;
        if (*dict).size == 128 as libc::c_int as size_t
            && (*(*dict).subdict).size != 128 as libc::c_int as size_t
            || (*dict).size != 128 as libc::c_int as size_t
                && (*(*dict).subdict).size == 128 as libc::c_int as size_t
        {
            skey = if (*(*dict).subdict).size == 128 as libc::c_int as size_t {
                xmlDictComputeFastKey(name, l as libc::c_int, (*(*dict).subdict).seed)
            } else {
                xmlDictComputeBigKey(name, l as libc::c_int, (*(*dict).subdict).seed)
                    as libc::c_ulong
            };
        } else {
            skey = okey;
        }
        key = skey.wrapping_rem((*(*dict).subdict).size);
        if (*((*(*dict).subdict).dict).offset(key as isize)).valid != 0 as libc::c_int {
            let mut tmp: xmlDictEntryPtr = 0 as *mut xmlDictEntry;
            tmp = &mut *((*(*dict).subdict).dict).offset(key as isize)
                as *mut _xmlDictEntry;
            while !((*tmp).next).is_null() {
                if (*tmp).okey == skey && (*tmp).len == l {
                    if memcmp(
                        (*tmp).name as *const libc::c_void,
                        name as *const libc::c_void,
                        l as libc::c_ulong,
                    ) == 0
                    {
                        return (*tmp).name;
                    }
                }
                nbi = nbi.wrapping_add(1);
                nbi;
                tmp = (*tmp).next;
            }
            if (*tmp).okey == skey && (*tmp).len == l {
                if memcmp(
                    (*tmp).name as *const libc::c_void,
                    name as *const libc::c_void,
                    l as libc::c_ulong,
                ) == 0
                {
                    return (*tmp).name;
                }
            }
        }
        key = okey.wrapping_rem((*dict).size);
    }
    ret = xmlDictAddString(dict, name, l);
    if ret.is_null() {
        return 0 as *const xmlChar;
    }
    if insert.is_null() {
        entry = &mut *((*dict).dict).offset(key as isize) as *mut _xmlDictEntry;
    } else {
        entry = xmlMalloc
            .expect(
                "non-null function pointer",
            )(::core::mem::size_of::<xmlDictEntry>() as libc::c_ulong)
            as xmlDictEntryPtr;
        if entry.is_null() {
            return 0 as *const xmlChar;
        }
    }
    (*entry).name = ret;
    (*entry).len = l;
    (*entry).next = 0 as *mut _xmlDictEntry;
    (*entry).valid = 1 as libc::c_int;
    (*entry).okey = okey;
    if !insert.is_null() {
        (*insert).next = entry;
    }
    (*dict).nbElems = ((*dict).nbElems).wrapping_add(1);
    (*dict).nbElems;
    if nbi > 3 as libc::c_int as libc::c_ulong
        && (*dict).size
            <= (8 as libc::c_int * 2048 as libc::c_int / 2 as libc::c_int
                / 3 as libc::c_int) as size_t
    {
        if xmlDictGrow(
            dict,
            (3 as libc::c_int * 2 as libc::c_int) as size_t * (*dict).size,
        ) != 0 as libc::c_int
        {
            return 0 as *const xmlChar;
        }
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictExists(
    mut dict: xmlDictPtr,
    mut name: *const xmlChar,
    mut len: libc::c_int,
) -> *const xmlChar {
    let mut key: libc::c_ulong = 0;
    let mut okey: libc::c_ulong = 0;
    let mut nbi: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut insert: xmlDictEntryPtr = 0 as *mut xmlDictEntry;
    let mut l: libc::c_uint = 0;
    if dict.is_null() || name.is_null() {
        return 0 as *const xmlChar;
    }
    if len < 0 as libc::c_int {
        l = strlen(name as *const libc::c_char) as libc::c_uint;
    } else {
        l = len as libc::c_uint;
    }
    if (*dict).limit > 0 as libc::c_int as size_t && l as size_t >= (*dict).limit
        || l > (2147483647 as libc::c_int / 2 as libc::c_int) as libc::c_uint
    {
        return 0 as *const xmlChar;
    }
    okey = if (*dict).size == 128 as libc::c_int as size_t {
        xmlDictComputeFastKey(name, l as libc::c_int, (*dict).seed)
    } else {
        xmlDictComputeBigKey(name, l as libc::c_int, (*dict).seed) as libc::c_ulong
    };
    key = okey.wrapping_rem((*dict).size);
    if (*((*dict).dict).offset(key as isize)).valid == 0 as libc::c_int {
        insert = 0 as xmlDictEntryPtr;
    } else {
        insert = &mut *((*dict).dict).offset(key as isize) as *mut _xmlDictEntry;
        while !((*insert).next).is_null() {
            if (*insert).okey == okey && (*insert).len == l {
                if memcmp(
                    (*insert).name as *const libc::c_void,
                    name as *const libc::c_void,
                    l as libc::c_ulong,
                ) == 0
                {
                    return (*insert).name;
                }
            }
            nbi = nbi.wrapping_add(1);
            nbi;
            insert = (*insert).next;
        }
        if (*insert).okey == okey && (*insert).len == l {
            if memcmp(
                (*insert).name as *const libc::c_void,
                name as *const libc::c_void,
                l as libc::c_ulong,
            ) == 0
            {
                return (*insert).name;
            }
        }
    }
    if !((*dict).subdict).is_null() {
        let mut skey: libc::c_ulong = 0;
        if (*dict).size == 128 as libc::c_int as size_t
            && (*(*dict).subdict).size != 128 as libc::c_int as size_t
            || (*dict).size != 128 as libc::c_int as size_t
                && (*(*dict).subdict).size == 128 as libc::c_int as size_t
        {
            skey = if (*(*dict).subdict).size == 128 as libc::c_int as size_t {
                xmlDictComputeFastKey(name, l as libc::c_int, (*(*dict).subdict).seed)
            } else {
                xmlDictComputeBigKey(name, l as libc::c_int, (*(*dict).subdict).seed)
                    as libc::c_ulong
            };
        } else {
            skey = okey;
        }
        key = skey.wrapping_rem((*(*dict).subdict).size);
        if (*((*(*dict).subdict).dict).offset(key as isize)).valid != 0 as libc::c_int {
            let mut tmp: xmlDictEntryPtr = 0 as *mut xmlDictEntry;
            tmp = &mut *((*(*dict).subdict).dict).offset(key as isize)
                as *mut _xmlDictEntry;
            while !((*tmp).next).is_null() {
                if (*tmp).okey == skey && (*tmp).len == l {
                    if memcmp(
                        (*tmp).name as *const libc::c_void,
                        name as *const libc::c_void,
                        l as libc::c_ulong,
                    ) == 0
                    {
                        return (*tmp).name;
                    }
                }
                nbi = nbi.wrapping_add(1);
                nbi;
                tmp = (*tmp).next;
            }
            if (*tmp).okey == skey && (*tmp).len == l {
                if memcmp(
                    (*tmp).name as *const libc::c_void,
                    name as *const libc::c_void,
                    l as libc::c_ulong,
                ) == 0
                {
                    return (*tmp).name;
                }
            }
        }
    }
    return 0 as *const xmlChar;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictQLookup(
    mut dict: xmlDictPtr,
    mut prefix: *const xmlChar,
    mut name: *const xmlChar,
) -> *const xmlChar {
    let mut okey: libc::c_ulong = 0;
    let mut key: libc::c_ulong = 0;
    let mut nbi: libc::c_ulong = 0 as libc::c_int as libc::c_ulong;
    let mut entry: xmlDictEntryPtr = 0 as *mut xmlDictEntry;
    let mut insert: xmlDictEntryPtr = 0 as *mut xmlDictEntry;
    let mut ret: *const xmlChar = 0 as *const xmlChar;
    let mut len: libc::c_uint = 0;
    let mut plen: libc::c_uint = 0;
    let mut l: libc::c_uint = 0;
    if dict.is_null() || name.is_null() {
        return 0 as *const xmlChar;
    }
    if prefix.is_null() {
        return xmlDictLookup(dict, name, -(1 as libc::c_int));
    }
    len = strlen(name as *const libc::c_char) as libc::c_uint;
    l = len;
    plen = strlen(prefix as *const libc::c_char) as libc::c_uint;
    len = len.wrapping_add((1 as libc::c_int as libc::c_uint).wrapping_add(plen));
    okey = if prefix.is_null() {
        if (*dict).size == 128 as libc::c_int as size_t {
            xmlDictComputeFastKey(name, l as libc::c_int, (*dict).seed)
        } else {
            xmlDictComputeBigKey(name, l as libc::c_int, (*dict).seed) as libc::c_ulong
        }
    } else if (*dict).size == 128 as libc::c_int as size_t {
        xmlDictComputeFastQKey(
            prefix,
            plen as libc::c_int,
            name,
            l as libc::c_int,
            (*dict).seed,
        )
    } else {
        xmlDictComputeBigQKey(
            prefix,
            plen as libc::c_int,
            name,
            l as libc::c_int,
            (*dict).seed,
        )
    };
    key = okey.wrapping_rem((*dict).size);
    if (*((*dict).dict).offset(key as isize)).valid == 0 as libc::c_int {
        insert = 0 as xmlDictEntryPtr;
    } else {
        insert = &mut *((*dict).dict).offset(key as isize) as *mut _xmlDictEntry;
        while !((*insert).next).is_null() {
            if (*insert).okey == okey && (*insert).len == len
                && xmlStrQEqual(prefix, name, (*insert).name) != 0
            {
                return (*insert).name;
            }
            nbi = nbi.wrapping_add(1);
            nbi;
            insert = (*insert).next;
        }
        if (*insert).okey == okey && (*insert).len == len
            && xmlStrQEqual(prefix, name, (*insert).name) != 0
        {
            return (*insert).name;
        }
    }
    if !((*dict).subdict).is_null() {
        let mut skey: libc::c_ulong = 0;
        if (*dict).size == 128 as libc::c_int as size_t
            && (*(*dict).subdict).size != 128 as libc::c_int as size_t
            || (*dict).size != 128 as libc::c_int as size_t
                && (*(*dict).subdict).size == 128 as libc::c_int as size_t
        {
            skey = if prefix.is_null() {
                if (*(*dict).subdict).size == 128 as libc::c_int as size_t {
                    xmlDictComputeFastKey(
                        name,
                        l as libc::c_int,
                        (*(*dict).subdict).seed,
                    )
                } else {
                    xmlDictComputeBigKey(name, l as libc::c_int, (*(*dict).subdict).seed)
                        as libc::c_ulong
                }
            } else if (*(*dict).subdict).size == 128 as libc::c_int as size_t {
                xmlDictComputeFastQKey(
                    prefix,
                    plen as libc::c_int,
                    name,
                    l as libc::c_int,
                    (*(*dict).subdict).seed,
                )
            } else {
                xmlDictComputeBigQKey(
                    prefix,
                    plen as libc::c_int,
                    name,
                    l as libc::c_int,
                    (*(*dict).subdict).seed,
                )
            };
        } else {
            skey = okey;
        }
        key = skey.wrapping_rem((*(*dict).subdict).size);
        if (*((*(*dict).subdict).dict).offset(key as isize)).valid != 0 as libc::c_int {
            let mut tmp: xmlDictEntryPtr = 0 as *mut xmlDictEntry;
            tmp = &mut *((*(*dict).subdict).dict).offset(key as isize)
                as *mut _xmlDictEntry;
            while !((*tmp).next).is_null() {
                if (*tmp).okey == skey && (*tmp).len == len
                    && xmlStrQEqual(prefix, name, (*tmp).name) != 0
                {
                    return (*tmp).name;
                }
                nbi = nbi.wrapping_add(1);
                nbi;
                tmp = (*tmp).next;
            }
            if (*tmp).okey == skey && (*tmp).len == len
                && xmlStrQEqual(prefix, name, (*tmp).name) != 0
            {
                return (*tmp).name;
            }
        }
        key = okey.wrapping_rem((*dict).size);
    }
    ret = xmlDictAddQString(dict, prefix, plen, name, l);
    if ret.is_null() {
        return 0 as *const xmlChar;
    }
    if insert.is_null() {
        entry = &mut *((*dict).dict).offset(key as isize) as *mut _xmlDictEntry;
    } else {
        entry = xmlMalloc
            .expect(
                "non-null function pointer",
            )(::core::mem::size_of::<xmlDictEntry>() as libc::c_ulong)
            as xmlDictEntryPtr;
        if entry.is_null() {
            return 0 as *const xmlChar;
        }
    }
    (*entry).name = ret;
    (*entry).len = len;
    (*entry).next = 0 as *mut _xmlDictEntry;
    (*entry).valid = 1 as libc::c_int;
    (*entry).okey = okey;
    if !insert.is_null() {
        (*insert).next = entry;
    }
    (*dict).nbElems = ((*dict).nbElems).wrapping_add(1);
    (*dict).nbElems;
    if nbi > 3 as libc::c_int as libc::c_ulong
        && (*dict).size
            <= (8 as libc::c_int * 2048 as libc::c_int / 2 as libc::c_int
                / 3 as libc::c_int) as size_t
    {
        xmlDictGrow(
            dict,
            (3 as libc::c_int * 2 as libc::c_int) as size_t * (*dict).size,
        );
    }
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictOwns(
    mut dict: xmlDictPtr,
    mut str: *const xmlChar,
) -> libc::c_int {
    let mut pool: xmlDictStringsPtr = 0 as *mut xmlDictStrings;
    if dict.is_null() || str.is_null() {
        return -(1 as libc::c_int);
    }
    pool = (*dict).strings;
    while !pool.is_null() {
        if str
            >= &mut *((*pool).array).as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut xmlChar as *const xmlChar
            && str <= (*pool).free as *const xmlChar
        {
            return 1 as libc::c_int;
        }
        pool = (*pool).next;
    }
    if !((*dict).subdict).is_null() {
        return xmlDictOwns((*dict).subdict, str);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictSize(mut dict: xmlDictPtr) -> libc::c_int {
    if dict.is_null() {
        return -(1 as libc::c_int);
    }
    if !((*dict).subdict).is_null() {
        return ((*dict).nbElems).wrapping_add((*(*dict).subdict).nbElems) as libc::c_int;
    }
    return (*dict).nbElems as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictSetLimit(
    mut dict: xmlDictPtr,
    mut limit: size_t,
) -> size_t {
    let mut ret: size_t = 0;
    if dict.is_null() {
        return 0 as libc::c_int as size_t;
    }
    ret = (*dict).limit;
    (*dict).limit = limit;
    return ret;
}
#[no_mangle]
pub unsafe extern "C" fn xmlDictGetUsage(mut dict: xmlDictPtr) -> size_t {
    let mut pool: xmlDictStringsPtr = 0 as *mut xmlDictStrings;
    let mut limit: size_t = 0 as libc::c_int as size_t;
    if dict.is_null() {
        return 0 as libc::c_int as size_t;
    }
    pool = (*dict).strings;
    while !pool.is_null() {
        limit = limit.wrapping_add((*pool).size);
        pool = (*pool).next;
    }
    return limit;
}
