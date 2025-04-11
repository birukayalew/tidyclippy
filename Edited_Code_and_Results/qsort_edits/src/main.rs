#![allow(
    dead_code,
    mutable_transmutes,
    non_camel_case_types,
    non_snake_case,
    non_upper_case_globals,
    unused_assignments,
    unused_mut
)]

use libc;

#[unsafe(no_mangle)]
pub unsafe extern "C" fn swap(mut a: *mut libc::c_int, mut b: *mut libc::c_int) {
    unsafe {
        let mut t: libc::c_int = *a;
        *a = *b;
        *b = t;
    }
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn partition(
    mut arr: *mut libc::c_int,
    mut low: libc::c_int,
    mut high: libc::c_int,
) -> libc::c_int {
    let mut pivot: libc::c_int = unsafe { *arr.offset(high as isize) };
    let mut i: libc::c_int = low.checked_sub(1).unwrap();
    let mut j: libc::c_int = low;
    while j <= high.checked_sub(1).unwrap() {
        if unsafe { *arr.offset(j as isize) } <= pivot {
            i = i.checked_add(1).unwrap();
            unsafe {
                swap(
                    &mut *arr.offset(i as isize),
                    &mut *arr.offset(j as isize),
                );
            }
        }
        j = j.checked_add(1).unwrap();
    }
    unsafe {
        swap(
            &mut *arr.offset(i.checked_add(1).unwrap() as isize),
            &mut *arr.offset(high as isize),
        );
    }
    i.checked_add(1).unwrap()
}

#[unsafe(no_mangle)]
pub unsafe extern "C" fn quickSort(
    mut arr: *mut libc::c_int,
    mut low: libc::c_int,
    mut high: libc::c_int,
) {
    if low < high {
        let mut i: libc::c_int = unsafe { partition(arr, low, high) };
        unsafe {
            quickSort(arr, low, i.checked_sub(1).unwrap());
            quickSort(arr, i.checked_add(1).unwrap(), high);
        }
    }
}

fn main() {}
