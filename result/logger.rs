// SAFETY: most functions in this module are marked as unsafe due to their args

use std::os::raw;

use crate::result::{GmshRawError, GmshRawResult};
use crate::sys;

pub unsafe fn gmsh_logger_write(
    message: *const ::std::os::raw::c_char,
    level: *const ::std::os::raw::c_char,
    ierr: *mut ::std::os::raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_logger_start(ierr: *mut ::std::os::raw::c_int) {
    todo!()
}

pub unsafe fn gmsh_logger_get(
    log: *mut *mut *mut ::std::os::raw::c_char,
    log_n: *mut usize,
    ierr: *mut ::std::os::raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_logger_stop(ierr: *mut ::std::os::raw::c_int) {
    todo!()
}

pub unsafe fn gmsh_logger_get_wall_time(ierr: *mut ::std::os::raw::c_int) -> f64 {
    todo!()
}

pub unsafe fn gmsh_logger_get_cpu_time(ierr: *mut ::std::os::raw::c_int) -> f64 {
    todo!()
}

pub unsafe fn gmsh_logger_get_memory(ierr: *mut ::std::os::raw::c_int) -> f64 {
    todo!()
}

pub unsafe fn gmsh_logger_get_total_memory(ierr: *mut ::std::os::raw::c_int) -> f64 {
    todo!()
}

pub unsafe fn gmsh_logger_get_last_error(
    error: *mut *mut ::std::os::raw::c_char,
    ierr: *mut ::std::os::raw::c_int,
) {
    todo!()
}
