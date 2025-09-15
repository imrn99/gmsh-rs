// SAFETY: most functions in this module are marked as unsafe due to their args

use std::os::raw;

use crate::result::{GmshRawError, GmshRawResult};
use crate::sys;

pub unsafe fn gmsh_view_add(
    name: *const raw::c_char,
    tag: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe { sys::gmshViewAdd(name, tag, &mut err) };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_view_remove(tag: raw::c_int) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe { sys::gmshViewRemove(tag, &mut err) };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_view_get_index(tag: raw::c_int) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe { sys::gmshViewGetIndex(tag, &mut err) };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_view_get_tags() -> GmshRawResult<(*mut raw::c_int, usize)> {
    let mut value: *mut i32 = std::ptr::null_mut();
    let mut count: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        sys::gmshViewGetTags(&mut value, &mut count, &mut err);
    };

    if err == 0 {
        Ok((value, count))
    } else {
        Err(GmshRawError(err))
    }
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
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshViewAddModelData(
            tag,
            step,
            model_name,
            data_type,
            tags,
            tags_n,
            data,
            data_n,
            data_nn,
            time,
            num_components,
            partition,
            ierr,
        );
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
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
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshViewAddHomogeneousModelData(
            tag,
            step,
            model_name,
            data_type,
            tags,
            tags_n,
            data,
            data_n,
            time,
            num_components,
            partition,
            ierr,
        )
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_view_get_model_data(
    tag: raw::c_int,
    step: raw::c_int,
) -> GmshRawResult<(
    *mut raw::c_char,
    (*mut usize, usize),
    (*mut *mut f64, *mut usize, usize),
    f64,
    raw::c_int,
)> {
    let mut data_type: *mut raw::c_char = std::ptr::null_mut();
    let mut tags: *mut usize = std::ptr::null_mut();
    let mut tags_n: usize = 0;
    let mut data: *mut *mut f64 = std::ptr::null_mut();
    let mut data_n: *mut usize = std::ptr::null_mut();
    let mut data_nn: usize = 0;
    let mut time: f64 = 0.0;
    let mut num_components: raw::c_int = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshViewGetModelData(
            tag,
            step,
            &mut data_type,
            &mut tags,
            &mut tags_n,
            &mut data,
            &mut data_n,
            &mut data_nn,
            &mut time,
            &mut num_components,
            ierr,
        )
    };

    if err == 0 {
        Ok((
            data_type,
            (tags, tags_n),
            (data, data_n, data_nn),
            time,
            num_components,
        ))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_view_get_homogeneous_model_data(
    tag: raw::c_int,
    step: raw::c_int,
) -> GmshRawResult<(
    *mut raw::c_char,
    (*mut usize, usize),
    (*mut f64, usize),
    f64,
    raw::c_int,
)> {
    let mut data_type: *mut raw::c_char = std::ptr::null_mut();
    let mut tags: *mut usize = std::ptr::null_mut();
    let mut tags_n: usize = 0;
    let mut data: *mut f64 = std::ptr::null_mut();
    let mut data_n: usize = 0;
    let mut time: f64 = 0.0;
    let mut num_components: raw::c_int = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshViewGetHomogeneousModelData(
            tag,
            step,
            &mut data_type,
            &mut tags,
            &mut tags_n,
            &mut data,
            &mut data_n,
            &mut time,
            &mut num_components,
            ierr,
        )
    };

    if err == 0 {
        Ok((
            data_type,
            (tags, tags_n),
            (data, data_n),
            time,
            num_components,
        ))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_view_add_list_data(
    tag: raw::c_int,
    data_type: *const raw::c_char,
    num_ele: raw::c_int,
    data: *const f64,
    data_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshViewAddListData(tag, data_type, num_ele, data, data_n, ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_view_get_list_data(
    tag: raw::c_int,
    return_adaptive: raw::c_int,
) -> GmshRawResult<(
    (*mut *mut raw::c_char, usize),
    (*mut raw::c_int, usize),
    (*mut *mut f64, *mut usize, usize),
)> {
    let mut data_type: *mut *mut raw::c_char = std::ptr::null_mut();
    let mut data_type_n: usize = 0;
    let mut num_elements: *mut raw::c_int = std::ptr::null_mut();
    let mut num_elements_n: usize = 0;
    let mut data: *mut *mut f64 = std::ptr::null_mut();
    let mut data_n: *mut usize = std::ptr::null_mut();
    let mut data_nn: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshViewGetListData(
            tag,
            &mut data_type,
            &mut data_type_n,
            &mut num_elements,
            &mut num_elements_n,
            &mut data,
            &mut data_n,
            &mut data_nn,
            return_adaptive,
            ierr,
        )
    };

    if err == 0 {
        Ok((
            (data_type, data_type_n),
            (num_elements, num_elements_n),
            (data, data_n, data_nn),
        ))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_view_add_list_data_string(
    tag: raw::c_int,
    coord: *const f64,
    coord_n: usize,
    data: *const *const raw::c_char,
    data_n: usize,
    style: *const *const raw::c_char,
    style_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshViewAddListDataString(tag, coord, coord_n, data, data_n, style, style_n, ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_view_get_list_data_strings(
    tag: raw::c_int,
    dim: raw::c_int,
) -> GmshRawResult<(
    (*mut f64, usize),
    (*mut *mut raw::c_char, usize),
    (*mut *mut raw::c_char, usize),
)> {
    let mut coord: *mut f64 = std::ptr::null_mut();
    let mut coord_n: usize = 0;
    let mut data: *mut *mut raw::c_char = std::ptr::null_mut();
    let mut data_n: usize = 0;
    let mut style: *mut *mut raw::c_char = std::ptr::null_mut();
    let mut style_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshViewGetListDataStrings(
            tag,
            dim,
            &mut coord,
            &mut coord_n,
            &mut data,
            &mut data_n,
            &mut style,
            &mut style_n,
            ierr,
        )
    };

    if err == 0 {
        Ok(((coord, coord_n), (data, data_n), (style, style_n)))
    } else {
        Err(GmshRawError(err))
    }
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
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshViewSetInterpolationMatrices(
            tag, type_, d, coef, coef_n, exp, exp_n, d_geo, coef_geo, coef_geo_n, exp_geo,
            exp_geo_n, ierr,
        )
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_view_add_alias(
    ref_tag: raw::c_int,
    copy_options: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshViewAddAlias(ref_tag, copy_options, tag, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_view_combine(
    what: *const raw::c_char,
    how: *const raw::c_char,
    remove: raw::c_int,
    copy_options: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshViewCombine(what, how, remove, copy_options, ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_view_probe(
    tag: raw::c_int,
    x: f64,
    y: f64,
    z: f64,
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
) -> GmshRawResult<(*mut f64, usize, f64)> {
    let mut values: *mut f64 = std::ptr::null_mut();
    let mut values_n: usize = 0;
    let mut distance: f64 = 0.0;
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshViewProbe(
            tag,
            x,
            y,
            z,
            &mut values,
            &mut values_n,
            &mut distance,
            step,
            num_comp,
            gradient,
            distance_max,
            x_elem_coord,
            x_elem_coord_n,
            y_elem_coord,
            y_elem_coord_n,
            z_elem_coord,
            z_elem_coord_n,
            dim,
            ierr,
        )
    };

    if err == 0 {
        Ok((values, values_n, distance))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_view_write(
    tag: raw::c_int,
    file_name: *const raw::c_char,
    append: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshViewWrite(tag, file_name, append, ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_view_set_visibility_per_window(
    tag: raw::c_int,
    value: raw::c_int,
    window_index: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshViewSetVisibilityPerWindow(tag, value, window_index, ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

/* gmsh/view/option namespace */

pub unsafe fn gmsh_view_option_set_number(
    tag: raw::c_int,
    name: *const raw::c_char,
    value: f64,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshViewOptionSetNumber(tag, name, value, ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_view_option_get_number(
    tag: raw::c_int,
    name: *const raw::c_char,
) -> GmshRawResult<f64> {
    let mut value: f64 = 0.0;
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshViewOptionGetNumber(tag, name, &mut value, ierr)
    };

    if err == 0 {
        Ok(value)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_view_option_set_string(
    tag: raw::c_int,
    name: *const raw::c_char,
    value: *const raw::c_char,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshViewOptionSetString(tag, name, value, ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_view_option_get_string(
    tag: raw::c_int,
    name: *const raw::c_char,
) -> GmshRawResult<*mut raw::c_char> {
    let mut value: *mut raw::c_char = std::ptr::null_mut();
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshViewOptionGetString(tag, name, &mut value, ierr)
    };

    if err == 0 {
        Ok(value)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_view_option_set_color(
    tag: raw::c_int,
    name: *const raw::c_char,
    [r, g, b, a]: [raw::c_int; 4],
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshViewOptionSetColor(tag, name, r, g, b, a, ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_view_option_get_color(
    tag: raw::c_int,
    name: *const raw::c_char,
) -> GmshRawResult<[raw::c_int; 4]> {
    let mut r: raw::c_int = 0;
    let mut g: raw::c_int = 0;
    let mut b: raw::c_int = 0;
    let mut a: raw::c_int = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshViewOptionGetColor(tag, name, &mut r, &mut g, &mut b, &mut a, ierr)
    };

    if err == 0 {
        Ok([r, g, b, a])
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_view_option_copy(ref_tag: raw::c_int, tag: raw::c_int) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshViewOptionCopy(ref_tag, tag, ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}
