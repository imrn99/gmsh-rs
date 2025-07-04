// SAFETY: most functions in this module are marked as unsafe due to their args

use std::os::raw;

use crate::result::{GmshRawError, GmshRawResult};
use crate::sys;

pub unsafe fn gmsh_view_add(
    name: *const raw::c_char,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> raw::c_int {
    todo!()
}

pub unsafe fn gmsh_view_remove(tag: raw::c_int, ierr: *mut raw::c_int) {
    todo!()
}

pub unsafe fn gmsh_view_get_index(tag: raw::c_int, ierr: *mut raw::c_int) -> raw::c_int {
    todo!()
}

pub unsafe fn gmsh_view_get_tags(
    tags: *mut *mut raw::c_int,
    tags_n: *mut usize,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_view_add_model_data(
    tag: raw::c_int,
    step: raw::c_int,
    model_name: *const raw::c_char,
    data_type: *const raw::c_char,
    tags: *const usize,
    tags_n: usize,
    data: *const *const f64,
    data_n: *const usize,
    data_nn: usize,
    time: f64,
    num_components: raw::c_int,
    partition: raw::c_int,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_view_add_homogeneous_model_data(
    tag: raw::c_int,
    step: raw::c_int,
    model_name: *const raw::c_char,
    data_type: *const raw::c_char,
    tags: *const usize,
    tags_n: usize,
    data: *const f64,
    data_n: usize,
    time: f64,
    num_components: raw::c_int,
    partition: raw::c_int,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_view_get_model_data(
    tag: raw::c_int,
    step: raw::c_int,
    data_type: *mut *mut raw::c_char,
    tags: *mut *mut usize,
    tags_n: *mut usize,
    data: *mut *mut *mut f64,
    data_n: *mut *mut usize,
    data_nn: *mut usize,
    time: *mut f64,
    num_components: *mut raw::c_int,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_view_get_homogeneous_model_data(
    tag: raw::c_int,
    step: raw::c_int,
    data_type: *mut *mut raw::c_char,
    tags: *mut *mut usize,
    tags_n: *mut usize,
    data: *mut *mut f64,
    data_n: *mut usize,
    time: *mut f64,
    num_components: *mut raw::c_int,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_view_add_list_data(
    tag: raw::c_int,
    data_type: *const raw::c_char,
    num_ele: raw::c_int,
    data: *const f64,
    data_n: usize,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_view_get_list_data(
    tag: raw::c_int,
    data_type: *mut *mut *mut raw::c_char,
    data_type_n: *mut usize,
    num_elements: *mut *mut raw::c_int,
    num_elements_n: *mut usize,
    data: *mut *mut *mut f64,
    data_n: *mut *mut usize,
    data_nn: *mut usize,
    return_adaptive: raw::c_int,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_view_add_list_data_string(
    tag: raw::c_int,
    coord: *const f64,
    coord_n: usize,
    data: *const *const raw::c_char,
    data_n: usize,
    style: *const *const raw::c_char,
    style_n: usize,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_view_get_list_data_strings(
    tag: raw::c_int,
    dim: raw::c_int,
    coord: *mut *mut f64,
    coord_n: *mut usize,
    data: *mut *mut *mut raw::c_char,
    data_n: *mut usize,
    style: *mut *mut *mut raw::c_char,
    style_n: *mut usize,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_view_set_interpolation_matrices(
    tag: raw::c_int,
    type_: *const raw::c_char,
    d: raw::c_int,
    coef: *const f64,
    coef_n: usize,
    exp: *const f64,
    exp_n: usize,
    d_geo: raw::c_int,
    coef_geo: *const f64,
    coef_geo_n: usize,
    exp_geo: *const f64,
    exp_geo_n: usize,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_view_add_alias(
    ref_tag: raw::c_int,
    copy_options: raw::c_int,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> raw::c_int {
    todo!()
}

pub unsafe fn gmsh_view_combine(
    what: *const raw::c_char,
    how: *const raw::c_char,
    remove: raw::c_int,
    copy_options: raw::c_int,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_view_probe(
    tag: raw::c_int,
    x: f64,
    y: f64,
    z: f64,
    values: *mut *mut f64,
    values_n: *mut usize,
    distance: *mut f64,
    step: raw::c_int,
    num_comp: raw::c_int,
    gradient: raw::c_int,
    distance_max: f64,
    x_elem_coord: *const f64,
    x_elem_coord_n: usize,
    y_elem_coord: *const f64,
    y_elem_coord_n: usize,
    z_elem_coord: *const f64,
    z_elem_coord_n: usize,
    dim: raw::c_int,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_view_write(
    tag: raw::c_int,
    file_name: *const raw::c_char,
    append: raw::c_int,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_view_set_visibility_per_window(
    tag: raw::c_int,
    value: raw::c_int,
    window_index: raw::c_int,
    ierr: *mut raw::c_int,
) {
    todo!()
}

/* gmsh/view/option namespace */

pub unsafe fn gmsh_view_option_set_number(
    tag: raw::c_int,
    name: *const raw::c_char,
    value: f64,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_view_option_get_number(
    tag: raw::c_int,
    name: *const raw::c_char,
    value: *mut f64,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_view_option_set_string(
    tag: raw::c_int,
    name: *const raw::c_char,
    value: *const raw::c_char,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_view_option_get_string(
    tag: raw::c_int,
    name: *const raw::c_char,
    value: *mut *mut raw::c_char,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_view_option_set_color(
    tag: raw::c_int,
    name: *const raw::c_char,
    r: raw::c_int,
    g: raw::c_int,
    b: raw::c_int,
    a: raw::c_int,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_view_option_get_color(
    tag: raw::c_int,
    name: *const raw::c_char,
    r: *mut raw::c_int,
    g: *mut raw::c_int,
    b: *mut raw::c_int,
    a: *mut raw::c_int,
    ierr: *mut raw::c_int,
) {
    todo!()
}

pub unsafe fn gmsh_view_option_copy(ref_tag: raw::c_int, tag: raw::c_int, ierr: *mut raw::c_int) {
    todo!()
}
