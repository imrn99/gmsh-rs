// SAFETY: most functions in this module are marked as unsafe due to their args

use std::os::raw;

use crate::result::{GmshRawError, GmshRawResult};
use crate::sys;

pub unsafe fn gmsh_plugin_set_number(
    name: *const raw::c_char,
    option: *const raw::c_char,
    value: f64,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_plugin_set_string(
    name: *const raw::c_char,
    option: *const raw::c_char,
    value: *const raw::c_char,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_plugin_run(name: *const raw::c_char, ierr: *mut raw::c_int) -> raw::c_int {
    todo!()
}
