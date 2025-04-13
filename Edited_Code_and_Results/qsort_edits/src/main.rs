#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]
#[no_mangle]
pub unsafe extern "C" fn swap(mut a: *mut libc::c_int, mut b: *mut libc::c_int) {
    let mut t: libc::c_int = *a;
    *a = *b;
    *b = t;
}
#[no_mangle]
pub unsafe extern "C" fn partition(
    mut arr: *mut libc::c_int,
    mut low: libc::c_int,
    mut high: libc::c_int,
) -> libc::c_int {
    let mut pivot: libc::c_int = *arr.offset(high as isize);
    let mut i: libc::c_int = low - 1 as libc::c_int;
    let mut j: libc::c_int = low;
    while j <= high - 1 as libc::c_int {
        if *arr.offset(j as isize) <= pivot {
            i += 1;
            i;
            swap(&mut *arr.offset(i as isize), &mut *arr.offset(j as isize));
        }
        j += 1;
        j;
    }
    swap(
        &mut *arr.offset((i + 1 as libc::c_int) as isize),
        &mut *arr.offset(high as isize),
    );
    return i + 1 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn quickSort(
    mut arr: *mut libc::c_int,
    mut low: libc::c_int,
    mut high: libc::c_int,
) {
    if low < high {
        let mut i: libc::c_int = partition(arr, low, high);
        quickSort(arr, low, i - 1 as libc::c_int);
        quickSort(arr, i + 1 as libc::c_int, high);
    }
}

fn main() {
    // Example dummy main just to compile
    println!("qsort module loaded");
}
