use ::libc;
extern "C" {
    pub type png_struct_def;
    pub type png_info_def;
    fn png_malloc(png_ptr: png_const_structrp, size: png_alloc_size_t) -> png_voidp;
    fn png_free(png_ptr: png_const_structrp, ptr: png_voidp);
    fn png_free_data(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        free_me: png_uint_32,
        num: libc::c_int,
    );
    fn png_error(png_ptr: png_const_structrp, error_message: png_const_charp) -> !;
    fn png_get_rowbytes(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
    ) -> size_t;
    fn png_set_rows(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        row_pointers: png_bytepp,
    );
    fn png_get_image_height(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
    ) -> png_uint_32;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type png_byte = libc::c_uchar;
pub type png_uint_32 = libc::c_uint;
pub type png_alloc_size_t = size_t;
pub type png_voidp = *mut libc::c_void;
pub type png_bytep = *mut png_byte;
pub type png_const_charp = *const libc::c_char;
pub type png_bytepp = *mut *mut png_byte;
pub type png_struct = png_struct_def;
pub type png_structp = *mut png_struct;
pub type png_info = png_info_def;
pub type png_infop = *mut png_info;
pub type png_const_structrp = *const png_struct;
pub type png_inforp = *mut png_info;
pub type png_const_inforp = *const png_info;
pub type pngx_alloc_size_t = png_alloc_size_t;
#[no_mangle]
pub unsafe extern "C" fn pngx_malloc_rows(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut filler: libc::c_int,
) -> png_bytepp {
    return pngx_malloc_rows_extended(
        png_ptr,
        info_ptr,
        0 as libc::c_int as pngx_alloc_size_t,
        filler,
    );
}
#[no_mangle]
pub unsafe extern "C" fn pngx_malloc_rows_extended(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut min_row_size: pngx_alloc_size_t,
    mut filler: libc::c_int,
) -> png_bytepp {
    let mut row_size: pngx_alloc_size_t = 0;
    let mut row: png_bytep = 0 as *mut png_byte;
    let mut rows: png_bytepp = 0 as *mut *mut png_byte;
    let mut height: png_uint_32 = 0;
    let mut i: png_uint_32 = 0;
    height = png_get_image_height(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
    );
    if height == 0 as libc::c_int as png_uint_32 {
        png_error(
            png_ptr as *const png_struct,
            b"Missing IHDR\0" as *const u8 as *const libc::c_char,
        );
    }
    row_size = png_get_rowbytes(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
    );
    if row_size == 0 as libc::c_int as pngx_alloc_size_t
        || height as pngx_alloc_size_t
            > (-(1 as libc::c_int) as pngx_alloc_size_t)
                .wrapping_div(::core::mem::size_of::<png_bytep>() as libc::c_ulong)
    {
        png_error(
            png_ptr as *const png_struct,
            b"Can't handle exceedingly large image dimensions\0" as *const u8
                as *const libc::c_char,
        );
    }
    if row_size < min_row_size {
        row_size = min_row_size;
    }
    png_free_data(
        png_ptr as *const png_struct,
        info_ptr,
        0x40 as libc::c_uint,
        0 as libc::c_int,
    );
    rows = png_malloc(
        png_ptr as *const png_struct,
        (height as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<png_bytep>() as libc::c_ulong),
    ) as png_bytepp;
    if rows.is_null() {
        return 0 as png_bytepp;
    }
    i = 0 as libc::c_int as png_uint_32;
    while i < height {
        row = png_malloc(png_ptr as *const png_struct, row_size) as png_bytep;
        if row.is_null() {
            while i > 0 as libc::c_int as png_uint_32 {
                i = i.wrapping_sub(1);
                png_free(
                    png_ptr as *const png_struct,
                    *rows.offset(i as isize) as png_voidp,
                );
            }
            png_free(png_ptr as *const png_struct, rows as png_voidp);
            return 0 as png_bytepp;
        }
        if filler >= 0 as libc::c_int {
            memset(row as *mut libc::c_void, filler, row_size);
        }
        let ref mut fresh0 = *rows.offset(i as isize);
        *fresh0 = row;
        i = i.wrapping_add(1);
        i;
    }
    png_set_rows(png_ptr as *const png_struct, info_ptr, rows);
    return rows;
}
