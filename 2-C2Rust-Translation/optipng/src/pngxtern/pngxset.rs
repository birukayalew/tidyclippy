use ::libc;
extern "C" {
    pub type png_struct_def;
    pub type png_info_def;
    fn png_get_IHDR(
        png_ptr: png_const_structrp,
        info_ptr: png_const_inforp,
        width: *mut png_uint_32,
        height: *mut png_uint_32,
        bit_depth: *mut libc::c_int,
        color_type: *mut libc::c_int,
        interlace_method: *mut libc::c_int,
        compression_method: *mut libc::c_int,
        filter_method: *mut libc::c_int,
    ) -> png_uint_32;
    fn png_set_IHDR(
        png_ptr: png_const_structrp,
        info_ptr: png_inforp,
        width: png_uint_32,
        height: png_uint_32,
        bit_depth: libc::c_int,
        color_type: libc::c_int,
        interlace_method: libc::c_int,
        compression_method: libc::c_int,
        filter_method: libc::c_int,
    );
}
pub type png_uint_32 = libc::c_uint;
pub type png_struct = png_struct_def;
pub type png_structp = *mut png_struct;
pub type png_info = png_info_def;
pub type png_infop = *mut png_info;
pub type png_const_structrp = *const png_struct;
pub type png_inforp = *mut png_info;
pub type png_const_inforp = *const png_info;
#[no_mangle]
pub unsafe extern "C" fn pngx_set_compression_type(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut compression_type: libc::c_int,
) {
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut bit_depth: libc::c_int = 0;
    let mut color_type: libc::c_int = 0;
    let mut interlace_type: libc::c_int = 0;
    let mut filter_type: libc::c_int = 0;
    let mut old_compression_type: libc::c_int = 0;
    if png_get_IHDR(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
        &mut width,
        &mut height,
        &mut bit_depth,
        &mut color_type,
        &mut interlace_type,
        &mut old_compression_type,
        &mut filter_type,
    ) == 0
    {
        return;
    }
    if compression_type == old_compression_type {
        return;
    }
    png_set_IHDR(
        png_ptr as *const png_struct,
        info_ptr,
        width,
        height,
        bit_depth,
        color_type,
        interlace_type,
        compression_type,
        filter_type,
    );
}
#[no_mangle]
pub unsafe extern "C" fn pngx_set_filter_type(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut filter_type: libc::c_int,
) {
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut bit_depth: libc::c_int = 0;
    let mut color_type: libc::c_int = 0;
    let mut interlace_type: libc::c_int = 0;
    let mut compression_type: libc::c_int = 0;
    let mut old_filter_type: libc::c_int = 0;
    if png_get_IHDR(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
        &mut width,
        &mut height,
        &mut bit_depth,
        &mut color_type,
        &mut interlace_type,
        &mut compression_type,
        &mut old_filter_type,
    ) == 0
    {
        return;
    }
    if filter_type == old_filter_type {
        return;
    }
    png_set_IHDR(
        png_ptr as *const png_struct,
        info_ptr,
        width,
        height,
        bit_depth,
        color_type,
        interlace_type,
        compression_type,
        filter_type,
    );
}
#[no_mangle]
pub unsafe extern "C" fn pngx_set_interlace_type(
    mut png_ptr: png_structp,
    mut info_ptr: png_infop,
    mut interlace_type: libc::c_int,
) {
    let mut width: png_uint_32 = 0;
    let mut height: png_uint_32 = 0;
    let mut bit_depth: libc::c_int = 0;
    let mut color_type: libc::c_int = 0;
    let mut compression_type: libc::c_int = 0;
    let mut filter_type: libc::c_int = 0;
    let mut old_interlace_type: libc::c_int = 0;
    if png_get_IHDR(
        png_ptr as *const png_struct,
        info_ptr as *const png_info,
        &mut width,
        &mut height,
        &mut bit_depth,
        &mut color_type,
        &mut old_interlace_type,
        &mut compression_type,
        &mut filter_type,
    ) == 0
    {
        return;
    }
    if interlace_type == old_interlace_type {
        return;
    }
    png_set_IHDR(
        png_ptr as *const png_struct,
        info_ptr,
        width,
        height,
        bit_depth,
        color_type,
        interlace_type,
        compression_type,
        filter_type,
    );
}
