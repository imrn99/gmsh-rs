// SAFETY: most functions in this module are marked as unsafe due to their args

use std::os::raw;

use crate::result::{GmshRawError, GmshRawResult};
use crate::sys;

pub unsafe fn gmsh_parser_get_names(
    names: *mut *mut *mut raw::c_char,
    names_n: *mut usize,
    search: *const raw::c_char,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_parser_set_number(
    name: *const raw::c_char,
    value: *const f64,
    value_n: usize,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_parser_set_string(
    name: *const raw::c_char,
    value: *const *const raw::c_char,
    value_n: usize,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_parser_get_number(
    name: *const raw::c_char,
    value: *mut *mut f64,
    value_n: *mut usize,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_parser_get_string(
    name: *const raw::c_char,
    value: *mut *mut *mut raw::c_char,
    value_n: *mut usize,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_parser_clear(name: *const raw::c_char, ierr: *mut raw::c_int) {
    todo!()
}

pub unsafe fn gmsh_parser_parse(file_name: *const raw::c_char, ierr: *mut raw::c_int) {
    todo!()
}
