// SAFETY: most functions in this module are marked as unsafe due to their args

use std::os::raw;

use crate::result::{GmshRawError, GmshRawResult};
use crate::sys;

pub unsafe fn gmsh_fltk_initialize(ierr: *mut raw::c_int) {
    todo!()
}

pub unsafe fn gmsh_fltk_finalize(ierr: *mut raw::c_int) {
    todo!()
}

pub unsafe fn gmsh_fltk_wait(time: f64, ierr: *mut raw::c_int) {
    todo!()
}

pub unsafe fn gmsh_fltk_update(ierr: *mut raw::c_int) {
    todo!()
}

pub unsafe fn gmsh_fltk_awake(action: *const raw::c_char, ierr: *mut raw::c_int) {
    todo!()
}

pub unsafe fn gmsh_fltk_lock(ierr: *mut raw::c_int) {
    todo!()
}

pub unsafe fn gmsh_fltk_unlock(ierr: *mut raw::c_int) {
    todo!()
}

pub unsafe fn gmsh_fltk_run(ierr: *mut raw::c_int) {
    todo!()
}

pub unsafe fn gmsh_fltk_is_available(ierr: *mut raw::c_int) -> raw::c_int {
    todo!()
}

pub unsafe fn gmsh_fltk_select_entities(
    dim_tags: *mut *mut raw::c_int,
    dim_tags_n: *mut usize,
    dim: raw::c_int,
    ierr: *mut raw::c_int,
) -> raw::c_int {
    todo!()
}

pub unsafe fn gmsh_fltk_select_elements(
    element_tags: *mut *mut usize,
    element_tags_n: *mut usize,
    ierr: *mut raw::c_int,
) -> raw::c_int {
    todo!()
}

pub unsafe fn gmsh_fltk_select_views(
    view_tags: *mut *mut raw::c_int,
    view_tags_n: *mut usize,
    ierr: *mut raw::c_int,
) -> raw::c_int {
    todo!()
}

pub unsafe fn gmsh_fltk_split_current_window(
    how: *const raw::c_char,
    ratio: f64,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_fltk_set_current_window(window_index: raw::c_int, ierr: *mut raw::c_int) {
    todo!()
}

pub unsafe fn gmsh_fltk_set_status_message(
    message: *const raw::c_char,
    graphics: raw::c_int,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_fltk_show_context_window(
    dim: raw::c_int,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_fltk_open_tree_item(name: *const raw::c_char, ierr: *mut raw::c_int) {
    todo!()
}

pub unsafe fn gmsh_fltk_close_tree_item(name: *const raw::c_char, ierr: *mut raw::c_int) {
    todo!()
}
