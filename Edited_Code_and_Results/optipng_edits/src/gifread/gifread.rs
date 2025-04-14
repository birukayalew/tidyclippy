use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn getc(__stream: *mut FILE) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn ferror(__stream: *mut FILE) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memcmp(
        _: *const libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GIFScreen {
    pub Width: libc::c_uint,
    pub Height: libc::c_uint,
    pub GlobalColorFlag: libc::c_uint,
    pub ColorResolution: libc::c_uint,
    pub SortFlag: libc::c_uint,
    pub GlobalNumColors: libc::c_uint,
    pub Background: libc::c_uint,
    pub PixelAspectRatio: libc::c_uint,
    pub GlobalColorTable: [libc::c_uchar; 768],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GIFImage {
    pub Screen: *mut GIFScreen,
    pub LeftPos: libc::c_uint,
    pub TopPos: libc::c_uint,
    pub Width: libc::c_uint,
    pub Height: libc::c_uint,
    pub LocalColorFlag: libc::c_uint,
    pub InterlaceFlag: libc::c_uint,
    pub SortFlag: libc::c_uint,
    pub LocalNumColors: libc::c_uint,
    pub LocalColorTable: [libc::c_uchar; 768],
    pub Rows: *mut *mut libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GIFExtension {
    pub Screen: *mut GIFScreen,
    pub Buffer: *mut libc::c_uchar,
    pub BufferSize: libc::c_uint,
    pub Label: libc::c_uchar,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct GIFGraphicCtlExt {
    pub DisposalMethod: libc::c_uint,
    pub InputFlag: libc::c_uint,
    pub TransparentFlag: libc::c_uint,
    pub DelayTime: libc::c_uint,
    pub Transparent: libc::c_uint,
}
#[no_mangle]
pub unsafe extern "C" fn GIFReadScreen(
    mut screen: *mut GIFScreen,
    mut stream: *mut FILE,
) {
    let mut buffer: [libc::c_uchar; 7] = [0; 7];
    ReadBytes(buffer.as_mut_ptr(), 6 as libc::c_int as libc::c_uint, stream);
    if memcmp(
        buffer.as_mut_ptr() as *const libc::c_void,
        b"GIF\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        3 as libc::c_int as libc::c_ulong,
    ) != 0 as libc::c_int
    {
        GIFError
            .expect(
                "non-null function pointer",
            )(b"Not a GIF file\0" as *const u8 as *const libc::c_char);
    }
    if memcmp(
        buffer.as_mut_ptr().offset(3 as libc::c_int as isize) as *const libc::c_void,
        b"87a\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        3 as libc::c_int as libc::c_ulong,
    ) != 0 as libc::c_int
        && memcmp(
            buffer.as_mut_ptr().offset(3 as libc::c_int as isize) as *const libc::c_void,
            b"89a\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            3 as libc::c_int as libc::c_ulong,
        ) != 0 as libc::c_int
    {
        GIFWarning
            .expect(
                "non-null function pointer",
            )(
            b"Invalid GIF version number, not \"87a\" or \"89a\"\0" as *const u8
                as *const libc::c_char,
        );
    }
    ReadBytes(buffer.as_mut_ptr(), 7 as libc::c_int as libc::c_uint, stream);
    (*screen)
        .Width = (*buffer
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize)
        .offset(0 as libc::c_int as isize) as libc::c_int
        + ((*buffer
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize)
            .offset(1 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int))
        as libc::c_uint;
    (*screen)
        .Height = (*buffer
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize)
        .offset(0 as libc::c_int as isize) as libc::c_int
        + ((*buffer
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize)
            .offset(1 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int))
        as libc::c_uint;
    (*screen)
        .GlobalColorFlag = (if buffer[4 as libc::c_int as usize] as libc::c_int
        & 0x80 as libc::c_int != 0
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }) as libc::c_uint;
    (*screen)
        .ColorResolution = (((buffer[4 as libc::c_int as usize] as libc::c_int
        & 0x70 as libc::c_int) >> 3 as libc::c_int) + 1 as libc::c_int) as libc::c_uint;
    (*screen)
        .SortFlag = (if buffer[4 as libc::c_int as usize] as libc::c_int
        & 0x8 as libc::c_int != 0
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }) as libc::c_uint;
    (*screen)
        .GlobalNumColors = ((2 as libc::c_int)
        << (buffer[4 as libc::c_int as usize] as libc::c_int & 0x7 as libc::c_int))
        as libc::c_uint;
    (*screen).Background = buffer[5 as libc::c_int as usize] as libc::c_uint;
    (*screen).PixelAspectRatio = buffer[6 as libc::c_int as usize] as libc::c_uint;
    if (*screen).GlobalColorFlag != 0 {
        ReadBytes(
            ((*screen).GlobalColorTable).as_mut_ptr(),
            (3 as libc::c_int as libc::c_uint).wrapping_mul((*screen).GlobalNumColors),
            stream,
        );
    }
    if (*screen).Width == 0 as libc::c_int as libc::c_uint
        || (*screen).Height == 0 as libc::c_int as libc::c_uint
    {
        GIFError
            .expect(
                "non-null function pointer",
            )(b"Invalid dimensions in GIF image\0" as *const u8 as *const libc::c_char);
    }
    if (*screen).Background > 0 as libc::c_int as libc::c_uint {
        if (*screen).GlobalColorFlag != 0
            && (*screen).Background >= (*screen).GlobalNumColors
            || (*screen).GlobalColorFlag == 0
        {
            (*screen).Background = 0 as libc::c_int as libc::c_uint;
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn GIFInitImage(
    mut image: *mut GIFImage,
    mut screen: *mut GIFScreen,
    mut rows: *mut *mut libc::c_uchar,
) {
    (*image).Screen = screen;
    (*image).Rows = rows;
}
#[no_mangle]
pub unsafe extern "C" fn GIFDestroyImage(mut image: *mut GIFImage) {}
#[no_mangle]
pub unsafe extern "C" fn GIFReadNextBlock(
    mut image: *mut GIFImage,
    mut ext: *mut GIFExtension,
    mut stream: *mut FILE,
) -> libc::c_int {
    let mut ch: libc::c_int = 0;
    let mut foundBogus: libc::c_int = 0;
    foundBogus = 0 as libc::c_int;
    loop {
        ch = GetByte(stream);
        match ch {
            44 => {
                GIFReadNextImage(image, stream);
                return ch;
            }
            33 => {
                GIFReadNextExtension(ext, stream);
                return ch;
            }
            59 => return ch,
            _ => {
                if foundBogus == 0 {
                    GIFWarning
                        .expect(
                            "non-null function pointer",
                        )(
                        b"Bogus data in GIF file\0" as *const u8 as *const libc::c_char,
                    );
                }
                foundBogus = 1 as libc::c_int;
            }
        }
    };
}
unsafe extern "C" fn GIFReadNextImage(mut image: *mut GIFImage, mut stream: *mut FILE) {
    let mut screen: *mut GIFScreen = 0 as *mut GIFScreen;
    let mut buffer: [libc::c_uchar; 9] = [0; 9];
    ReadBytes(buffer.as_mut_ptr(), 9 as libc::c_int as libc::c_uint, stream);
    if image.is_null() {
        GIFSkipDataBlocks(stream);
        return;
    }
    (*image)
        .LeftPos = (*buffer
        .as_mut_ptr()
        .offset(0 as libc::c_int as isize)
        .offset(0 as libc::c_int as isize) as libc::c_int
        + ((*buffer
            .as_mut_ptr()
            .offset(0 as libc::c_int as isize)
            .offset(1 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int))
        as libc::c_uint;
    (*image)
        .TopPos = (*buffer
        .as_mut_ptr()
        .offset(2 as libc::c_int as isize)
        .offset(0 as libc::c_int as isize) as libc::c_int
        + ((*buffer
            .as_mut_ptr()
            .offset(2 as libc::c_int as isize)
            .offset(1 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int))
        as libc::c_uint;
    (*image)
        .Width = (*buffer
        .as_mut_ptr()
        .offset(4 as libc::c_int as isize)
        .offset(0 as libc::c_int as isize) as libc::c_int
        + ((*buffer
            .as_mut_ptr()
            .offset(4 as libc::c_int as isize)
            .offset(1 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int))
        as libc::c_uint;
    (*image)
        .Height = (*buffer
        .as_mut_ptr()
        .offset(6 as libc::c_int as isize)
        .offset(0 as libc::c_int as isize) as libc::c_int
        + ((*buffer
            .as_mut_ptr()
            .offset(6 as libc::c_int as isize)
            .offset(1 as libc::c_int as isize) as libc::c_int) << 8 as libc::c_int))
        as libc::c_uint;
    (*image)
        .LocalColorFlag = (if buffer[8 as libc::c_int as usize] as libc::c_int
        & 0x80 as libc::c_int != 0
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }) as libc::c_uint;
    (*image)
        .InterlaceFlag = (if buffer[8 as libc::c_int as usize] as libc::c_int
        & 0x40 as libc::c_int != 0
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }) as libc::c_uint;
    (*image)
        .SortFlag = (if buffer[8 as libc::c_int as usize] as libc::c_int
        & 0x20 as libc::c_int != 0
    {
        1 as libc::c_int
    } else {
        0 as libc::c_int
    }) as libc::c_uint;
    (*image)
        .LocalNumColors = (if (*image).LocalColorFlag != 0 {
        (2 as libc::c_int)
            << (buffer[8 as libc::c_int as usize] as libc::c_int & 0x7 as libc::c_int)
    } else {
        0 as libc::c_int
    }) as libc::c_uint;
    if (*image).LocalColorFlag != 0 {
        ReadBytes(
            ((*image).LocalColorTable).as_mut_ptr(),
            (3 as libc::c_int as libc::c_uint).wrapping_mul((*image).LocalNumColors),
            stream,
        );
    }
    screen = (*image).Screen;
    if (*image).Width == 0 as libc::c_int as libc::c_uint
        || (*image).Height == 0 as libc::c_int as libc::c_uint
        || ((*image).LeftPos).wrapping_add((*image).Width) > (*screen).Width
        || ((*image).TopPos).wrapping_add((*image).Height) > (*screen).Height
    {
        GIFError
            .expect(
                "non-null function pointer",
            )(b"Invalid dimensions in GIF image\0" as *const u8 as *const libc::c_char);
    }
    GIFReadImageData(image, stream);
}
unsafe extern "C" fn GIFReadImageData(mut image: *mut GIFImage, mut stream: *mut FILE) {
    let mut minCodeSize: libc::c_int = 0;
    let mut rows: *mut *mut libc::c_uchar = 0 as *mut *mut libc::c_uchar;
    let mut width: libc::c_uint = 0;
    let mut height: libc::c_uint = 0;
    let mut interlaced: libc::c_uint = 0;
    let mut colors: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut numColors: libc::c_uint = 0;
    let mut xpos: libc::c_uint = 0;
    let mut ypos: libc::c_uint = 0;
    let mut pass: libc::c_int = 0;
    let mut val: libc::c_int = 0;
    minCodeSize = GetByte(stream);
    if minCodeSize >= 12 as libc::c_int {
        GIFError
            .expect(
                "non-null function pointer",
            )(b"Invalid LZW code size\0" as *const u8 as *const libc::c_char);
    }
    if LZWDecodeByte(1 as libc::c_int, minCodeSize, stream) < 0 as libc::c_int {
        GIFError
            .expect(
                "non-null function pointer",
            )(b"Error decoding GIF image\0" as *const u8 as *const libc::c_char);
    }
    rows = (*image).Rows;
    if rows.is_null() {
        GIFSkipDataBlocks(stream);
        return;
    }
    width = (*image).Width;
    height = (*image).Height;
    interlaced = (*image).InterlaceFlag;
    GIFGetColorTable(&mut colors, &mut numColors, image);
    ypos = 0 as libc::c_int as libc::c_uint;
    xpos = ypos;
    pass = 0 as libc::c_int;
    loop {
        val = LZWDecodeByte(0 as libc::c_int, minCodeSize, stream);
        if !(val >= 0 as libc::c_int) {
            break;
        }
        if val as libc::c_uint >= numColors {
            GIFWarning
                .expect(
                    "non-null function pointer",
                )(
                b"Pixel value out of range in GIF image\0" as *const u8
                    as *const libc::c_char,
            );
            val = numColors.wrapping_sub(1 as libc::c_int as libc::c_uint)
                as libc::c_int;
        }
        *(*rows.offset(ypos as isize)).offset(xpos as isize) = val as libc::c_uchar;
        xpos = xpos.wrapping_add(1);
        if xpos == width {
            xpos = 0 as libc::c_int as libc::c_uint;
            if interlaced != 0 {
                match pass {
                    0 | 1 => {
                        ypos = ypos.wrapping_add(8 as libc::c_int as libc::c_uint);
                    }
                    2 => {
                        ypos = ypos.wrapping_add(4 as libc::c_int as libc::c_uint);
                    }
                    3 => {
                        ypos = ypos.wrapping_add(2 as libc::c_int as libc::c_uint);
                    }
                    _ => {}
                }
                if ypos >= height {
                    pass += 1;
                    match pass {
                        1 => {
                            ypos = 4 as libc::c_int as libc::c_uint;
                        }
                        2 => {
                            ypos = 2 as libc::c_int as libc::c_uint;
                        }
                        3 => {
                            ypos = 1 as libc::c_int as libc::c_uint;
                        }
                        _ => {
                            break;
                        }
                    }
                }
            } else {
                ypos = ypos.wrapping_add(1);
                ypos;
            }
        }
        if ypos >= height {
            break;
        }
    }
    while LZWDecodeByte(0 as libc::c_int, minCodeSize, stream) >= 0 as libc::c_int {}
}
static mut DataBlockSize: libc::c_int = 0 as libc::c_int;
unsafe extern "C" fn GIFReadDataBlock(
    mut buffer: *mut libc::c_uchar,
    mut stream: *mut FILE,
) -> libc::c_int {
    let mut count: libc::c_int = 0;
    count = GetByte(stream);
    DataBlockSize = count;
    if count > 0 as libc::c_int {
        ReadBytes(buffer, count as libc::c_uint, stream);
    }
    return count;
}
unsafe extern "C" fn GIFSkipDataBlocks(mut stream: *mut FILE) {
    let mut count: libc::c_int = 0;
    let mut buffer: [libc::c_uchar; 256] = [0; 256];
    loop {
        count = GetByte(stream);
        if count > 0 as libc::c_int {
            ReadBytes(buffer.as_mut_ptr(), count as libc::c_uint, stream);
        } else {
            return
        }
    };
}
unsafe extern "C" fn LZWGetCode(
    mut code_size: libc::c_int,
    mut init_flag: libc::c_int,
    mut stream: *mut FILE,
) -> libc::c_int {
    static mut buffer: [libc::c_uchar; 280] = [0; 280];
    static mut curbit: libc::c_int = 0;
    static mut lastbit: libc::c_int = 0;
    static mut done: libc::c_int = 0;
    static mut last_byte: libc::c_int = 0;
    let mut count: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut ret: libc::c_int = 0;
    if init_flag != 0 {
        curbit = 0 as libc::c_int;
        lastbit = 0 as libc::c_int;
        last_byte = 2 as libc::c_int;
        done = 0 as libc::c_int;
        return 0 as libc::c_int;
    }
    if curbit + code_size >= lastbit {
        if done != 0 {
            if curbit >= lastbit {
                GIFError
                    .expect(
                        "non-null function pointer",
                    )(
                    b"Ran off the end of input bits in LZW decoding\0" as *const u8
                        as *const libc::c_char,
                );
            }
            return -(1 as libc::c_int);
        }
        buffer[0 as libc::c_int
            as usize] = buffer[(last_byte - 2 as libc::c_int) as usize];
        buffer[1 as libc::c_int
            as usize] = buffer[(last_byte - 1 as libc::c_int) as usize];
        count = GIFReadDataBlock(
            &mut *buffer.as_mut_ptr().offset(2 as libc::c_int as isize),
            stream,
        );
        if count == 0 as libc::c_int {
            done = 1 as libc::c_int;
        }
        last_byte = 2 as libc::c_int + count;
        curbit = curbit - lastbit + 16 as libc::c_int;
        lastbit = (2 as libc::c_int + count) * 8 as libc::c_int;
    }
    if code_size != 0
        && (curbit + code_size - 1 as libc::c_int) as size_t / 8 as libc::c_int as size_t
            >= ::core::mem::size_of::<[libc::c_uchar; 280]>() as libc::c_ulong
    {
        GIFError
            .expect(
                "non-null function pointer",
            )(b"Malformed GIF (CVE-2023-43907)\0" as *const u8 as *const libc::c_char);
    }
    ret = 0 as libc::c_int;
    i = curbit;
    j = 0 as libc::c_int;
    while j < code_size {
        ret
            |= ((buffer[(i / 8 as libc::c_int) as usize] as libc::c_int
                & (1 as libc::c_int) << i % 8 as libc::c_int != 0 as libc::c_int)
                as libc::c_int) << j;
        i += 1;
        i;
        j += 1;
        j;
    }
    curbit += code_size;
    return ret;
}
unsafe extern "C" fn LZWDecodeByte(
    mut init_flag: libc::c_int,
    mut input_code_size: libc::c_int,
    mut stream: *mut FILE,
) -> libc::c_int {
    static mut fresh: libc::c_int = 0 as libc::c_int;
    static mut code_size: libc::c_int = 0;
    static mut set_code_size: libc::c_int = 0;
    static mut max_code: libc::c_int = 0;
    static mut max_code_size: libc::c_int = 0;
    static mut firstcode: libc::c_int = 0;
    static mut oldcode: libc::c_int = 0;
    static mut clear_code: libc::c_int = 0;
    static mut end_code: libc::c_int = 0;
    static mut table: [[libc::c_int; 4096]; 2] = [[0; 4096]; 2];
    static mut stack: [libc::c_int; 8192] = [0; 8192];
    static mut sp: *mut libc::c_int = 0 as *const libc::c_int as *mut libc::c_int;
    let mut code: libc::c_int = 0;
    let mut incode: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    if init_flag != 0 {
        fresh = 1 as libc::c_int;
        set_code_size = input_code_size;
        code_size = set_code_size + 1 as libc::c_int;
        clear_code = (1 as libc::c_int) << set_code_size;
        end_code = clear_code + 1 as libc::c_int;
        max_code_size = 2 as libc::c_int * clear_code;
        max_code = clear_code + 2 as libc::c_int;
        LZWGetCode(0 as libc::c_int, 1 as libc::c_int, stream);
        i = 0 as libc::c_int;
        while i < clear_code {
            table[0 as libc::c_int as usize][i as usize] = 0 as libc::c_int;
            table[1 as libc::c_int as usize][i as usize] = i;
            i += 1;
            i;
        }
        while i <= ((1 as libc::c_int) << 12 as libc::c_int) - 1 as libc::c_int {
            table[0 as libc::c_int as usize][i as usize] = 0 as libc::c_int;
            table[1 as libc::c_int as usize][i as usize] = 0 as libc::c_int;
            i += 1;
            i;
        }
        sp = stack.as_mut_ptr();
        return 0 as libc::c_int;
    } else if fresh != 0 {
        fresh = 0 as libc::c_int;
        loop {
            oldcode = LZWGetCode(code_size, 0 as libc::c_int, stream);
            firstcode = oldcode;
            if !(firstcode == clear_code) {
                break;
            }
        }
        return firstcode;
    }
    if sp > stack.as_mut_ptr() {
        sp = sp.offset(-1);
        return *sp;
    }
    loop {
        code = LZWGetCode(code_size, 0 as libc::c_int, stream);
        if !(code >= 0 as libc::c_int) {
            break;
        }
        if code == clear_code {
            i = 0 as libc::c_int;
            while i < clear_code {
                table[0 as libc::c_int as usize][i as usize] = 0 as libc::c_int;
                table[1 as libc::c_int as usize][i as usize] = i;
                i += 1;
                i;
            }
            while i <= ((1 as libc::c_int) << 12 as libc::c_int) - 1 as libc::c_int {
                table[0 as libc::c_int as usize][i as usize] = 0 as libc::c_int;
                table[1 as libc::c_int as usize][i as usize] = 0 as libc::c_int;
                i += 1;
                i;
            }
            code_size = set_code_size + 1 as libc::c_int;
            max_code_size = 2 as libc::c_int * clear_code;
            max_code = clear_code + 2 as libc::c_int;
            sp = stack.as_mut_ptr();
            oldcode = LZWGetCode(code_size, 0 as libc::c_int, stream);
            firstcode = oldcode;
            return firstcode;
        } else if code == end_code {
            let mut count: libc::c_int = 0;
            let mut buffer: [libc::c_uchar; 260] = [0; 260];
            if DataBlockSize == 0 as libc::c_int {
                return -(2 as libc::c_int);
            }
            loop {
                count = GIFReadDataBlock(buffer.as_mut_ptr(), stream);
                if !(count > 0 as libc::c_int) {
                    break;
                }
            }
            return -(2 as libc::c_int);
        }
        incode = code;
        if code >= max_code {
            let fresh0 = sp;
            sp = sp.offset(1);
            *fresh0 = firstcode;
            code = oldcode;
        }
        while code >= clear_code {
            let fresh1 = sp;
            sp = sp.offset(1);
            *fresh1 = table[1 as libc::c_int as usize][code as usize];
            if code == table[0 as libc::c_int as usize][code as usize]
                || sp.offset_from(stack.as_mut_ptr()) as libc::c_long as size_t
                    >= (::core::mem::size_of::<[libc::c_int; 8192]>() as libc::c_ulong)
                        .wrapping_div(
                            ::core::mem::size_of::<libc::c_int>() as libc::c_ulong,
                        )
            {
                GIFError
                    .expect(
                        "non-null function pointer",
                    )(
                    b"Circular dependency found in LZW table\0" as *const u8
                        as *const libc::c_char,
                );
            }
            code = table[0 as libc::c_int as usize][code as usize];
        }
        firstcode = table[1 as libc::c_int as usize][code as usize];
        let fresh2 = sp;
        sp = sp.offset(1);
        *fresh2 = firstcode;
        code = max_code;
        if code <= ((1 as libc::c_int) << 12 as libc::c_int) - 1 as libc::c_int {
            table[0 as libc::c_int as usize][code as usize] = oldcode;
            table[1 as libc::c_int as usize][code as usize] = firstcode;
            max_code += 1;
            max_code;
            if max_code >= max_code_size
                && max_code_size
                    <= ((1 as libc::c_int) << 12 as libc::c_int) - 1 as libc::c_int
            {
                max_code_size *= 2 as libc::c_int;
                code_size += 1;
                code_size;
            }
        }
        oldcode = incode;
        if sp > stack.as_mut_ptr() {
            sp = sp.offset(-1);
            return *sp;
        }
    }
    return code;
}
static mut DefaultColorTable: [libc::c_uchar; 24] = [
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    255 as libc::c_int as libc::c_uchar,
    0 as libc::c_int as libc::c_uchar,
];
#[no_mangle]
pub unsafe extern "C" fn GIFGetColorTable(
    mut colors: *mut *mut libc::c_uchar,
    mut numColors: *mut libc::c_uint,
    mut image: *mut GIFImage,
) {
    let mut screen: *mut GIFScreen = 0 as *mut GIFScreen;
    if (*image).LocalColorFlag != 0 {
        *colors = ((*image).LocalColorTable).as_mut_ptr();
        *numColors = (*image).LocalNumColors;
        return;
    }
    screen = (*image).Screen;
    if (*screen).GlobalColorFlag != 0 {
        *colors = ((*screen).GlobalColorTable).as_mut_ptr();
        *numColors = (*screen).GlobalNumColors;
        return;
    }
    *colors = DefaultColorTable.as_mut_ptr();
    *numColors = (::core::mem::size_of::<[libc::c_uchar; 24]>() as libc::c_ulong)
        .wrapping_div(3 as libc::c_int as libc::c_ulong) as libc::c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn GIFInitExtension(
    mut ext: *mut GIFExtension,
    mut screen: *mut GIFScreen,
    mut initBufferSize: libc::c_uint,
) {
    let mut newBuffer: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    (*ext).Screen = screen;
    if initBufferSize > 0 as libc::c_int as libc::c_uint {
        newBuffer = malloc(initBufferSize as libc::c_ulong) as *mut libc::c_uchar;
        if newBuffer.is_null() {
            ErrorAlloc();
        }
        (*ext).Buffer = newBuffer;
        (*ext).BufferSize = initBufferSize;
    } else {
        (*ext).Buffer = 0 as *mut libc::c_uchar;
        (*ext).BufferSize = 0 as libc::c_int as libc::c_uint;
    };
}
#[no_mangle]
pub unsafe extern "C" fn GIFDestroyExtension(mut ext: *mut GIFExtension) {
    free((*ext).Buffer as *mut libc::c_void);
}
unsafe extern "C" fn GIFReadNextExtension(
    mut ext: *mut GIFExtension,
    mut stream: *mut FILE,
) {
    let mut newBuffer: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    let mut newBufferSize: libc::c_uint = 0;
    let mut offset: libc::c_uint = 0;
    let mut len: libc::c_uint = 0;
    let mut count: libc::c_int = 0;
    let mut label: libc::c_int = 0;
    label = GetByte(stream);
    if ext.is_null() {
        GIFSkipDataBlocks(stream);
        return;
    }
    (*ext).Label = label as libc::c_uchar;
    offset = 0 as libc::c_int as libc::c_uint;
    len = (*ext).BufferSize;
    loop {
        if len < 255 as libc::c_int as libc::c_uint {
            newBufferSize = ((*ext).BufferSize)
                .wrapping_add(1024 as libc::c_int as libc::c_uint);
            newBuffer = realloc(
                (*ext).Buffer as *mut libc::c_void,
                newBufferSize as libc::c_ulong,
            ) as *mut libc::c_uchar;
            if newBuffer.is_null() {
                ErrorAlloc();
            }
            (*ext).BufferSize = newBufferSize;
            (*ext).Buffer = newBuffer;
            len = len.wrapping_add(1024 as libc::c_int as libc::c_uint);
        }
        count = GIFReadDataBlock(((*ext).Buffer).offset(offset as isize), stream);
        if count == 0 as libc::c_int {
            break;
        }
        offset = offset.wrapping_add(count as libc::c_uint);
        len = len.wrapping_sub(count as libc::c_uint);
    };
}
#[no_mangle]
pub unsafe extern "C" fn GIFGetGraphicCtl(
    mut graphicExt: *mut GIFGraphicCtlExt,
    mut ext: *mut GIFExtension,
) {
    let mut buffer: *mut libc::c_uchar = 0 as *mut libc::c_uchar;
    if (*ext).Label as libc::c_int != 0xf9 as libc::c_int {
        GIFWarning
            .expect(
                "non-null function pointer",
            )(
            b"Not a graphic control extension in GIF file\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    if (*ext).BufferSize < 4 as libc::c_int as libc::c_uint {
        GIFWarning
            .expect(
                "non-null function pointer",
            )(
            b"Broken graphic control extension in GIF file\0" as *const u8
                as *const libc::c_char,
        );
        return;
    }
    buffer = (*ext).Buffer;
    (*graphicExt)
        .DisposalMethod = (*buffer.offset(0 as libc::c_int as isize) as libc::c_int
        >> 2 as libc::c_int & 0x7 as libc::c_int) as libc::c_uint;
    (*graphicExt)
        .InputFlag = (*buffer.offset(0 as libc::c_int as isize) as libc::c_int
        >> 1 as libc::c_int & 0x1 as libc::c_int) as libc::c_uint;
    (*graphicExt)
        .TransparentFlag = (*buffer.offset(0 as libc::c_int as isize) as libc::c_int
        & 0x1 as libc::c_int) as libc::c_uint;
    (*graphicExt)
        .DelayTime = (*buffer
        .offset(1 as libc::c_int as isize)
        .offset(0 as libc::c_int as isize) as libc::c_int
        + ((*buffer.offset(1 as libc::c_int as isize).offset(1 as libc::c_int as isize)
            as libc::c_int) << 8 as libc::c_int)) as libc::c_uint;
    (*graphicExt)
        .Transparent = *buffer.offset(3 as libc::c_int as isize) as libc::c_uint;
}
unsafe extern "C" fn GetByte(mut stream: *mut FILE) -> libc::c_int {
    let mut ch: libc::c_int = 0;
    ch = getc(stream);
    if ch == -(1 as libc::c_int) {
        ErrorRead(stream);
    }
    return ch;
}
unsafe extern "C" fn ReadBytes(
    mut buffer: *mut libc::c_uchar,
    mut count: libc::c_uint,
    mut stream: *mut FILE,
) {
    if fread(
        buffer as *mut libc::c_void,
        count as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        stream,
    ) != 1 as libc::c_int as libc::c_ulong
    {
        ErrorRead(stream);
    }
}
unsafe extern "C" fn ErrorAlloc() {
    GIFError
        .expect(
            "non-null function pointer",
        )(b"Out of memory in GIF decoder\0" as *const u8 as *const libc::c_char);
}
unsafe extern "C" fn ErrorRead(mut stream: *mut FILE) {
    if ferror(stream) != 0 {
        GIFError
            .expect(
                "non-null function pointer",
            )(b"Error reading GIF file\0" as *const u8 as *const libc::c_char);
    } else {
        GIFError
            .expect(
                "non-null function pointer",
            )(b"Unexpected end of GIF file\0" as *const u8 as *const libc::c_char);
    };
}
unsafe extern "C" fn DefaultError(mut message: *const libc::c_char) {
    fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, message);
    exit(1 as libc::c_int);
}
unsafe extern "C" fn DefaultWarning(mut message: *const libc::c_char) {
    fprintf(stderr, b"%s\n\0" as *const u8 as *const libc::c_char, message);
}
#[no_mangle]
pub static mut GIFError: Option::<unsafe extern "C" fn(*const libc::c_char) -> ()> = unsafe {
    Some(DefaultError as unsafe extern "C" fn(*const libc::c_char) -> ())
};
#[no_mangle]
pub static mut GIFWarning: Option::<unsafe extern "C" fn(*const libc::c_char) -> ()> = unsafe {
    Some(DefaultWarning as unsafe extern "C" fn(*const libc::c_char) -> ())
};
