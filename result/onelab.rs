// SAFETY: most functions in this module are marked as unsafe due to their args

use std::os::raw;

use crate::result::{GmshRawError, GmshRawResult};
use crate::sys;

pub unsafe fn gmsh_onelab_set(
    data: *const raw::c_char,
    format: *const raw::c_char,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_onelab_get(
    data: *mut *mut raw::c_char,
    name: *const raw::c_char,
    format: *const raw::c_char,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_onelab_get_names(
    names: *mut *mut *mut raw::c_char,
    names_n: *mut usize,
    search: *const raw::c_char,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_onelab_set_number(
    name: *const raw::c_char,
    value: *const f64,
    value_n: usize,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_onelab_set_string(
    name: *const raw::c_char,
    value: *const *const raw::c_char,
    value_n: usize,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_onelab_get_number(
    name: *const raw::c_char,
    value: *mut *mut f64,
    value_n: *mut usize,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_onelab_get_string(
    name: *const raw::c_char,
    value: *mut *mut *mut raw::c_char,
    value_n: *mut usize,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_onelab_get_changed(
    name: *const raw::c_char,
    ierr: *mut raw::c_int,
) -> raw::c_int {
    todo!()
}

pub unsafe fn gmsh_onelab_set_changed(
    name: *const raw::c_char,
    value: raw::c_int,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_onelab_clear(name: *const raw::c_char, ierr: *mut raw::c_int) {
    todo!()
}

pub unsafe fn gmsh_onelab_run(
    name: *const raw::c_char,
    command: *const raw::c_char,
    ierr: *mut raw::c_int,
) {
    todo!()
}
