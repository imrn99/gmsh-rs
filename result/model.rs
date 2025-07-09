// SAFETY: most functions in this module are marked as unsafe due to their args

use std::os::raw;

use crate::result::{GmshRawError, GmshRawResult};
use crate::sys;

pub unsafe fn gmsh_model_add(name: *const raw::c_char) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelAdd(name, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_remove() -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelRemove(ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_list() -> GmshRawResult<(*const *const raw::c_char, usize)> {
    let mut value: *mut *mut raw::c_char = std::ptr::null_mut();
    let mut count: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let names = (&mut value) as *mut *mut *mut raw::c_char;
        let names_n = &mut count as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelList(names, names_n, ierr);
    };

    if err == 0 {
        Ok((value as *const *const raw::c_char, count))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_current() -> GmshRawResult<*mut raw::c_char> {
    let mut value: *mut raw::c_char = std::ptr::null_mut();
    let mut err: raw::c_int = 0;

    unsafe {
        let name = (&mut value) as *mut *mut raw::c_char;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetCurrent(name, ierr);
    };

    if err == 0 {
        Ok(value)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_set_current(name: *const raw::c_char) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelSetCurrent(name, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_file_name() -> GmshRawResult<*mut raw::c_char> {
    let mut value: *mut raw::c_char = std::ptr::null_mut();
    let mut err: raw::c_int = 0;

    unsafe {
        let file_name = (&mut value) as *mut *mut raw::c_char;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetCurrent(file_name, ierr);
    };

    if err == 0 {
        Ok(value)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_set_file_name(file_name: *const raw::c_char) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelSetFileName(file_name, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_entities(
    dim: raw::c_int,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut value: *mut raw::c_int = std::ptr::null_mut();
    let mut count: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let dim_tags = (&mut value) as *mut *mut raw::c_int;
        let dim_tags_n = &mut count as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetEntities(dim_tags, dim_tags_n, dim, ierr);
    };

    if err == 0 {
        Ok((value, count))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_set_entity_name(
    dim: raw::c_int,
    tag: raw::c_int,
    name: *const raw::c_char,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelSetEntityName(dim, tag, name, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_entity_name(
    dim: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<*mut raw::c_char> {
    let mut value: *mut raw::c_char = std::ptr::null_mut();
    let mut err: raw::c_int = 0;

    unsafe {
        let name = (&mut value) as *mut *mut raw::c_char;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetEntityName(dim, tag, name, ierr);
    };

    if err == 0 {
        Ok(value)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_remove_entity_name(name: *const raw::c_char) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelSetFileName(name, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_physical_groups(
    dim: raw::c_int,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut value: *mut raw::c_int = std::ptr::null_mut();
    let mut count: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let dim_tags = (&mut value) as *mut *mut raw::c_int;
        let dim_tags_n = &mut count as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetEntities(dim_tags, dim_tags_n, dim, ierr);
    };

    if err == 0 {
        Ok((value, count))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_entities_for_physical_group(
    dim: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut value: *mut raw::c_int = std::ptr::null_mut();
    let mut count: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let tags = (&mut value) as *mut *mut raw::c_int;
        let tags_n = &mut count as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetEntitiesForPhysicalGroup(dim, tag, tags, tags_n, ierr);
    };

    if err == 0 {
        Ok((value, count))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_entities_for_physical_name(
    name: *const raw::c_char,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut value: *mut raw::c_int = std::ptr::null_mut();
    let mut count: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let dim_tags = (&mut value) as *mut *mut raw::c_int;
        let dim_tags_n = &mut count as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetEntitiesForPhysicalName(name, dim_tags, dim_tags_n, ierr);
    };

    if err == 0 {
        Ok((value, count))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_physical_groups_for_entity(
    dim: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut value: *mut raw::c_int = std::ptr::null_mut();
    let mut count: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let physical_tags = (&mut value) as *mut *mut raw::c_int;
        let physical_tags_n = &mut count as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetPhysicalGroupsForEntity(dim, tag, physical_tags, physical_tags_n, ierr);
    };

    if err == 0 {
        Ok((value, count))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_add_physical_group(
    dim: raw::c_int,
    tags: *const raw::c_int,
    tags_n: usize,
    tag: raw::c_int,
    name: *const raw::c_char,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelAddPhysicalGroup(dim, tags, tags_n, tag, name, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_remove_physical_groups(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelRemovePhysicalGroups(dim_tags, dim_tags_n, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_set_physical_name(
    dim: raw::c_int,
    tag: raw::c_int,
    name: *const raw::c_char,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelSetPhysicalName(dim, tag, name, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_physical_name(
    dim: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<*mut raw::c_char> {
    let mut value: *mut raw::c_char = std::ptr::null_mut();
    let mut err: raw::c_int = 0;

    unsafe {
        let name = (&mut value) as *mut *mut raw::c_char;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetPhysicalName(dim, tag, name, ierr);
    };

    if err == 0 {
        Ok(value)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_remove_physical_name(name: *const raw::c_char) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelRemovePhysicalName(name, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_set_tag(
    dim: raw::c_int,
    tag: raw::c_int,
    new_tag: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelSetTag(dim, tag, new_tag, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_boundary(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    combined: raw::c_int,
    oriented: raw::c_int,
    recursive: raw::c_int,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut value: *mut raw::c_int = std::ptr::null_mut();
    let mut count: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let out_dim_tags = (&mut value) as *mut *mut raw::c_int;
        let out_dim_tags_n = (&mut count) as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetBoundary(
            dim_tags,
            dim_tags_n,
            out_dim_tags,
            out_dim_tags_n,
            combined,
            oriented,
            recursive,
            ierr,
        );
    };

    if err == 0 {
        Ok((value, count))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_adjacencies(dim: raw::c_int, tag: raw::c_int) -> GmshRawResult<()> {
    let mut up: *mut raw::c_int = std::ptr::null_mut();
    let mut up_count: usize = 0;
    let mut down: *mut raw::c_int = std::ptr::null_mut();
    let mut down_count: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let upward = &mut up as *mut *mut raw::c_int;
        let upward_n = &mut up_count as *mut usize;
        let downward = &mut down as *mut *mut raw::c_int;
        let downward_n = &mut down_count as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetAdjacencies(dim, tag, upward, upward_n, downward, downward_n, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_entities_in_bounding_box(
    xmin: f64,
    ymin: f64,
    zmin: f64,
    xmax: f64,
    ymax: f64,
    zmax: f64,
    dim: raw::c_int,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut value: *mut raw::c_int = std::ptr::null_mut();
    let mut count: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let dim_tags = (&mut value) as *mut *mut raw::c_int;
        let dim_tags_n = (&mut count) as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetEntitiesInBoundingBox(
            xmin, ymin, zmin, xmax, ymax, zmax, dim_tags, dim_tags_n, dim, ierr,
        );
    };

    if err == 0 {
        Ok((value, count))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_bounding_box(
    dim: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<[(f64, f64); 3]> {
    let mut xmin: f64 = 0.0;
    let mut ymin: f64 = 0.0;
    let mut zmin: f64 = 0.0;
    let mut xmax: f64 = 0.0;
    let mut ymax: f64 = 0.0;
    let mut zmax: f64 = 0.0;
    let mut err: raw::c_int = 0;

    unsafe {
        let xmin: *mut f64 = &mut xmin as *mut f64;
        let ymin: *mut f64 = &mut ymin as *mut f64;
        let zmin: *mut f64 = &mut zmin as *mut f64;
        let xmax: *mut f64 = &mut xmax as *mut f64;
        let ymax: *mut f64 = &mut ymax as *mut f64;
        let zmax: *mut f64 = &mut zmax as *mut f64;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetBoundingBox(dim, tag, xmin, ymin, zmin, xmax, ymax, zmax, ierr);
    };

    if err == 0 {
        Ok([(xmin, xmax), (ymin, ymax), (zmin, zmax)])
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_dimension() -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetDimension(ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_add_discrete_entity(
    dim: raw::c_int,
    tag: raw::c_int,
    boundary: *const raw::c_int,
    boundary_n: usize,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelAddDiscreteEntity(dim, tag, boundary, boundary_n, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_remove_entities(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    recursive: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelRemoveEntities(dim_tags, dim_tags_n, recursive, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_type(
    dim: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<*mut raw::c_char> {
    let mut value: *mut raw::c_char = std::ptr::null_mut();
    let mut err: raw::c_int = 0;

    unsafe {
        let entity_type = &mut value as *mut *mut raw::c_char;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetType(dim, tag, entity_type, ierr);
    };

    if err == 0 {
        Ok(value)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_parent(
    dim: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<(raw::c_int, raw::c_int)> {
    let mut v1: raw::c_int = 0;
    let mut v2: raw::c_int = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let parent_dim = &mut v1 as *mut raw::c_int;
        let parent_tag = &mut v2 as *mut raw::c_int;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetParent(dim, tag, parent_dim, parent_tag, ierr);
    };

    if err == 0 {
        Ok((v1, v2))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_number_of_partitions() -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetNumberOfPartitions(ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_partitions(
    dim: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut value: *mut raw::c_int = std::ptr::null_mut();
    let mut count: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let partitions = &mut value as *mut *mut raw::c_int;
        let partitions_n = &mut count as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetPartitions(dim, tag, partitions, partitions_n, ierr);
    };

    if err == 0 {
        Ok((value, count))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_value(
    dim: raw::c_int,
    tag: raw::c_int,
    parametric_coord: *const f64,
    parametric_coord_n: usize,
) -> GmshRawResult<(*const f64, usize)> {
    let mut value: *mut f64 = std::ptr::null_mut();
    let mut count: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let coord = &mut value as *mut *mut f64;
        let coord_n = &mut count as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetValue(
            dim,
            tag,
            parametric_coord,
            parametric_coord_n,
            coord,
            coord_n,
            ierr,
        );
    };

    if err == 0 {
        Ok((value, count))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_derivative(
    dim: raw::c_int,
    tag: raw::c_int,
    parametric_coord: *const f64,
    parametric_coord_n: usize,
) -> GmshRawResult<(*const f64, usize)> {
    let mut value: *mut f64 = std::ptr::null_mut();
    let mut count: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let derivatives = &mut value as *mut *mut f64;
        let derivatives_n = &mut count as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetDerivative(
            dim,
            tag,
            parametric_coord,
            parametric_coord_n,
            derivatives,
            derivatives_n,
            ierr,
        );
    };

    if err == 0 {
        Ok((value, count))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_second_derivative(
    dim: raw::c_int,
    tag: raw::c_int,
    parametric_coord: *const f64,
    parametric_coord_n: usize,
) -> GmshRawResult<(*const f64, usize)> {
    let mut value: *mut f64 = std::ptr::null_mut();
    let mut count: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let derivatives = &mut value as *mut *mut f64;
        let derivatives_n = &mut count as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetSecondDerivative(
            dim,
            tag,
            parametric_coord,
            parametric_coord_n,
            derivatives,
            derivatives_n,
            ierr,
        );
    };

    if err == 0 {
        Ok((value, count))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_curvature(
    dim: raw::c_int,
    tag: raw::c_int,
    parametric_coord: *const f64,
    parametric_coord_n: usize,
) -> GmshRawResult<(*const f64, usize)> {
    let mut value: *mut f64 = std::ptr::null_mut();
    let mut count: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let curvatures = &mut value as *mut *mut f64;
        let curvatures_n = &mut count as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetCurvature(
            dim,
            tag,
            parametric_coord,
            parametric_coord_n,
            curvatures,
            curvatures_n,
            ierr,
        );
    };

    if err == 0 {
        Ok((value, count))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_principal_curvatures(
    tag: raw::c_int,
    parametric_coord: *const f64,
    parametric_coord_n: usize,
) -> GmshRawResult<[(*const f64, usize); 4]> {
    let mut curvature_max: *mut f64 = std::ptr::null_mut();
    let mut curvature_max_n: usize = 0;
    let mut curvature_min: *mut f64 = std::ptr::null_mut();
    let mut curvature_min_n: usize = 0;
    let mut direction_max: *mut f64 = std::ptr::null_mut();
    let mut direction_max_n: usize = 0;
    let mut direction_min: *mut f64 = std::ptr::null_mut();
    let mut direction_min_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let curvature_max = &mut curvature_max as *mut *mut f64;
        let curvature_max_n = &mut curvature_max_n as *mut usize;
        let curvature_min = &mut curvature_min as *mut *mut f64;
        let curvature_min_n = &mut curvature_min_n as *mut usize;
        let direction_max = &mut direction_max as *mut *mut f64;
        let direction_max_n = &mut direction_max_n as *mut usize;
        let direction_min = &mut direction_min as *mut *mut f64;
        let direction_min_n = &mut direction_min_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetPrincipalCurvatures(
            tag,
            parametric_coord,
            parametric_coord_n,
            curvature_max,
            curvature_max_n,
            curvature_min,
            curvature_min_n,
            direction_max,
            direction_max_n,
            direction_min,
            direction_min_n,
            ierr,
        );
    };

    if err == 0 {
        Ok([
            (curvature_max, curvature_max_n),
            (curvature_min, curvature_min_n),
            (direction_max, direction_max_n),
            (direction_min, direction_min_n),
        ])
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_normal(
    tag: raw::c_int,
    parametric_coord: *const f64,
    parametric_coord_n: usize,
) -> GmshRawResult<(*const f64, usize)> {
    let mut value: *mut f64 = std::ptr::null_mut();
    let mut count: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let normals = &mut value as *mut *mut f64;
        let normals_n = &mut count as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetNormal(
            tag,
            parametric_coord,
            parametric_coord_n,
            normals,
            normals_n,
            ierr,
        );
    };

    if err == 0 {
        Ok((value, count))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_parametrization(
    dim: raw::c_int,
    tag: raw::c_int,
    coord: *const f64,
    coord_n: usize,
) -> GmshRawResult<(*const f64, usize)> {
    let mut value: *mut f64 = std::ptr::null_mut();
    let mut count: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let parametric_coord = &mut value as *mut *mut f64;
        let parametric_coord_n = &mut count as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetParametrization(
            dim,
            tag,
            coord,
            coord_n,
            parametric_coord,
            parametric_coord_n,
            ierr,
        );
    };

    if err == 0 {
        Ok((value, count))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_parametrization_bounds(
    dim: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<[(*const f64, usize); 2]> {
    let mut max: *mut f64 = std::ptr::null_mut();
    let mut max_n: usize = 0;
    let mut min: *mut f64 = std::ptr::null_mut();
    let mut min_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let max = &mut max as *mut *mut f64;
        let max_n = &mut max_n as *mut usize;
        let min = &mut min as *mut *mut f64;
        let min_n = &mut min_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetParametrizationBounds(dim, tag, max, max_n, min, min_n, ierr);
    };

    if err == 0 {
        Ok([(max, max_n), (min, min_n)])
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_is_inside(
    dim: raw::c_int,
    tag: raw::c_int,
    coord: *const f64,
    coord_n: usize,
    parametric: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelIsInside(dim, tag, coord, coord_n, parametric, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_closest_point(
    dim: raw::c_int,
    tag: raw::c_int,
    coord: *const f64,
    coord_n: usize,
) -> GmshRawResult<[(*const f64, usize); 2]> {
    let mut closest_coord: *mut f64 = std::ptr::null_mut();
    let mut closest_coord_n: usize = 0;
    let mut parametric_coord: *mut f64 = std::ptr::null_mut();
    let mut parametric_coord_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let closest_coord = &mut closest_coord as *mut *mut f64;
        let closest_coord_n = &mut closest_coord_n as *mut usize;
        let parametric_coord = &mut parametric_coord as *mut *mut f64;
        let parametric_coord_n = &mut parametric_coord_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetClosestPoint(
            dim,
            tag,
            coord,
            coord_n,
            closest_coord,
            closest_coord_n,
            parametric_coord,
            parametric_coord_n,
            ierr,
        );
    };

    if err == 0 {
        Ok([
            (closest_coord, closest_coord_n),
            (parametric_coord, parametric_coord_n),
        ])
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_reparametrize_on_surface(
    dim: raw::c_int,
    tag: raw::c_int,
    parametric_coord: *const f64,
    parametric_coord_n: usize,
    surface_tag: raw::c_int,
    which: raw::c_int,
) -> GmshRawResult<(*mut f64, usize)> {
    let mut value: *mut f64 = std::ptr::null_mut();
    let mut count: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let surface_parametric_coord = &mut value as *mut *mut f64;
        let surface_parametric_coord_n = &mut count as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelReparametrizeOnSurface(
            dim,
            tag,
            parametric_coord,
            parametric_coord_n,
            surface_tag,
            surface_parametric_coord,
            surface_parametric_coord_n,
            which,
            ierr,
        );
    };

    if err == 0 {
        Ok((value, count))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_set_visibility(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    value: raw::c_int,
    recursive: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelSetVisibility(dim_tags, dim_tags_n, value, recursive, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_visibility(
    dim: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut value: raw::c_int = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let value = &mut value as *mut raw::c_int;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetVisibility(dim, tag, value, ierr);
    };

    if err == 0 {
        Ok(value)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_set_visibility_per_window(
    value: raw::c_int,
    window_index: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelSetVisibilityPerWindow(value, window_index, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_set_color(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    r: raw::c_int,
    g: raw::c_int,
    b: raw::c_int,
    a: raw::c_int,
    recursive: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelSetColor(dim_tags, dim_tags_n, r, g, b, a, recursive, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_color(
    dim: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<[raw::c_int; 4]> {
    let mut r: raw::c_int = 0;
    let mut g: raw::c_int = 0;
    let mut b: raw::c_int = 0;
    let mut a: raw::c_int = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let r = &mut r as *mut raw::c_int;
        let g = &mut g as *mut raw::c_int;
        let b = &mut b as *mut raw::c_int;
        let a = &mut a as *mut raw::c_int;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetColor(dim, tag, r, g, b, a, ierr);
    };

    if err == 0 {
        Ok([r, g, b, a])
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_set_coordinates(
    tag: raw::c_int,
    x: f64,
    y: f64,
    z: f64,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelSetCoordinates(tag, x, y, z, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_set_attribute(
    name: *const raw::c_char,
    values: *const *const raw::c_char,
    values_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelSetAttribute(name, values, values_n, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_attribute(
    name: *const raw::c_char,
) -> GmshRawResult<(*const *const raw::c_char, usize)> {
    let mut value: *mut *mut raw::c_char = std::ptr::null_mut();
    let mut count: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let values = &mut value as *mut *mut *mut raw::c_char;
        let values_n = &mut count as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetAttribute(name, values, values_n, ierr);
    };

    if err == 0 {
        Ok((value as *const *const raw::c_char, count))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_get_attribute_names() -> GmshRawResult<(*const *const raw::c_char, usize)>
{
    let mut value: *mut *mut raw::c_char = std::ptr::null_mut();
    let mut count: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let values = &mut value as *mut *mut *mut raw::c_char;
        let values_n = &mut count as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGetAttributeNames(values, values_n, ierr);
    };

    if err == 0 {
        Ok((value as *const *const raw::c_char, count))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_remove_attribute(name: *const raw::c_char) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelRemoveAttribute(name, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

/* gmsh/model/mesh namespace */

pub unsafe fn gmsh_model_mesh_generate(dim: raw::c_int) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGenerate(dim, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_partition(
    num_part: raw::c_int,
    element_tags: *const usize,
    element_tags_n: usize,
    partitions: *const raw::c_int,
    partitions_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshPartition(
            num_part,
            element_tags,
            element_tags_n,
            partitions,
            partitions_n,
            ierr,
        );
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_unpartition() -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshUnpartition(ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_optimize(
    method: *const raw::c_char,
    force: raw::c_int,
    niter: raw::c_int,
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshOptimize(method, force, niter, dim_tags, dim_tags_n, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_recombine() -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshRecombine(ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_refine() -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshRefine(ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_set_order(order: raw::c_int) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshSetOrder(order, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_last_entity_error() -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut value: *mut raw::c_int = std::ptr::null_mut();
    let mut count: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let dim_tags = &mut value as *mut *mut raw::c_int;
        let dim_tags_n = &mut count as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetLastEntityError(dim_tags, dim_tags_n, ierr);
    };

    if err == 0 {
        Ok((value, count))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_last_node_error() -> GmshRawResult<(*mut usize, usize)> {
    let mut value: *mut usize = std::ptr::null_mut();
    let mut count: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let node_tags = &mut value as *mut *mut usize;
        let node_tags_n = &mut count as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetLastNodeError(node_tags, node_tags_n, ierr);
    };

    if err == 0 {
        Ok((value, count))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_clear(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshClear(dim_tags, dim_tags_n, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_remove_elements(
    dim: raw::c_int,
    tag: raw::c_int,
    element_tags: *const usize,
    element_tags_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshRemoveElements(dim, tag, element_tags, element_tags_n, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_reverse(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshReverse(dim_tags, dim_tags_n, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_reverse_elements(
    element_tags: *const usize,
    element_tags_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshReverseElements(element_tags, element_tags_n, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_affine_transform(
    affine_transform: *const f64,
    affine_transform_n: usize,
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshAffineTransform(
            affine_transform,
            affine_transform_n,
            dim_tags,
            dim_tags_n,
            ierr,
        );
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_nodes(
    dim: raw::c_int,
    tag: raw::c_int,
    include_boundary: raw::c_int,
    return_parametric_coord: raw::c_int,
) -> GmshRawResult<(
    (*const usize, usize),
    (*const f64, usize),
    (*const f64, usize),
)> {
    let mut node_tags: *mut usize = std::ptr::null_mut();
    let mut node_tags_n: usize = 0;
    let mut coord: *mut f64 = std::ptr::null_mut();
    let mut coord_n: usize = 0;
    let mut parametric_coord: *mut f64 = std::ptr::null_mut();
    let mut parametric_coord_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let node_tags = &mut node_tags as *mut *mut usize;
        let node_tags_n = &mut node_tags_n as *mut usize;
        let coord = &mut coord as *mut *mut f64;
        let coord_n = &mut coord_n as *mut usize;
        let parametric_coord = &mut parametric_coord as *mut *mut f64;
        let parametric_coord_n = &mut parametric_coord_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetNodes(
            node_tags,
            node_tags_n,
            coord,
            coord_n,
            parametric_coord,
            parametric_coord_n,
            dim,
            tag,
            include_boundary,
            return_parametric_coord,
            ierr,
        );
    };

    if err == 0 {
        Ok((
            (node_tags, node_tags_n),
            (coord, coord_n),
            (parametric_coord, parametric_coord_n),
        ))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_nodes_by_element_type(
    element_type: raw::c_int,
    tag: raw::c_int,
    return_parametric_coord: raw::c_int,
) -> GmshRawResult<(
    (*const usize, usize),
    (*const f64, usize),
    (*const f64, usize),
)> {
    let mut node_tags: *mut usize = std::ptr::null_mut();
    let mut node_tags_n: usize = 0;
    let mut coord: *mut f64 = std::ptr::null_mut();
    let mut coord_n: usize = 0;
    let mut parametric_coord: *mut f64 = std::ptr::null_mut();
    let mut parametric_coord_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let node_tags = &mut node_tags as *mut *mut usize;
        let node_tags_n = &mut node_tags_n as *mut usize;
        let coord = &mut coord as *mut *mut f64;
        let coord_n = &mut coord_n as *mut usize;
        let parametric_coord = &mut parametric_coord as *mut *mut f64;
        let parametric_coord_n = &mut parametric_coord_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetNodesByElementType(
            element_type,
            node_tags,
            node_tags_n,
            coord,
            coord_n,
            parametric_coord,
            parametric_coord_n,
            tag,
            return_parametric_coord,
            ierr,
        );
    };

    if err == 0 {
        Ok((
            (node_tags, node_tags_n),
            (coord, coord_n),
            (parametric_coord, parametric_coord_n),
        ))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_node(
    node_tag: usize,
) -> GmshRawResult<(
    (*const f64, usize),
    (*const f64, usize),
    raw::c_int,
    raw::c_int,
)> {
    let mut coord: *mut f64 = std::ptr::null_mut();
    let mut coord_n: usize = 0;
    let mut parametric_coord: *mut f64 = std::ptr::null_mut();
    let mut parametric_coord_n: usize = 0;
    let mut dim: raw::c_int = 0;
    let mut tag: raw::c_int = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let coord = &mut coord as *mut *mut f64;
        let coord_n = &mut coord_n as *mut usize;
        let parametric_coord = &mut parametric_coord as *mut *mut f64;
        let parametric_coord_n = &mut parametric_coord_n as *mut usize;
        let dim = &mut dim as *mut raw::c_int;
        let tag = &mut tag as *mut raw::c_int;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetNode(
            node_tag,
            coord,
            coord_n,
            parametric_coord,
            parametric_coord_n,
            dim,
            tag,
            ierr,
        );
    };

    if err == 0 {
        Ok((
            (coord, coord_n),
            (parametric_coord, parametric_coord_n),
            dim,
            tag,
        ))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_set_node(
    node_tag: usize,
    coord: *const f64,
    coord_n: usize,
    parametric_coord: *const f64,
    parametric_coord_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshSetNode(
            node_tag,
            coord,
            coord_n,
            parametric_coord,
            parametric_coord_n,
            ierr,
        );
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_rebuild_node_cache(
    only_if_necessary: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshRebuildNodeCache(only_if_necessary, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_rebuild_element_cache(
    only_if_necessary: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshRebuildElementCache(only_if_necessary, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_nodes_for_physical_group(
    dim: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<((*const usize, usize), (*mut f64, usize))> {
    let mut node_tags: *mut usize = std::ptr::null_mut();
    let mut node_tags_n: usize = 0;
    let mut coord: *mut f64 = std::ptr::null_mut();
    let mut coord_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let node_tags = &mut node_tags as *mut *mut usize;
        let node_tags_n = &mut node_tags_n as *mut usize;
        let coord = &mut coord as *mut *mut f64;
        let coord_n = &mut coord_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetNodesForPhysicalGroup(
            dim,
            tag,
            node_tags,
            node_tags_n,
            coord,
            coord_n,
            ierr,
        );
    };

    if err == 0 {
        Ok(((node_tags, node_tags_n), (coord, coord_n)))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_max_node_tag() -> GmshRawResult<usize> {
    let mut value: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let max_tag = &mut value as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetMaxNodeTag(max_tag, ierr);
    };

    if err == 0 {
        Ok(value)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_add_nodes(
    dim: raw::c_int,
    tag: raw::c_int,
    node_tags: *const usize,
    node_tags_n: usize,
    coord: *const f64,
    coord_n: usize,
    parametric_coord: *const f64,
    parametric_coord_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshAddNodes(
            dim,
            tag,
            node_tags,
            node_tags_n,
            coord,
            coord_n,
            parametric_coord,
            parametric_coord_n,
            ierr,
        );
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_reclassify_nodes() -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshReclassifyNodes(ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_relocate_nodes(
    dim: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshRelocateNodes(dim, tag, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_elements(
    dim: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<(
    (*const raw::c_int, usize),                 // element types
    (*const *const usize, *const usize, usize), // element tags
    (*const *const usize, *const usize, usize), // node tags
)> {
    let mut element_types: *mut raw::c_int = std::ptr::null_mut();
    let mut element_types_n: usize = 0;
    let mut element_tags: *mut *mut usize = std::ptr::null_mut();
    let mut element_tags_n: *mut usize = std::ptr::null_mut();
    let mut element_tags_nn: usize = 0;
    let mut node_tags: *mut *mut usize = std::ptr::null_mut();
    let mut node_tags_n: *mut usize = std::ptr::null_mut();
    let mut node_tags_nn: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let element_types = &mut element_types as *mut *mut raw::c_int;
        let element_types_n = &mut element_types_n as *mut usize;
        let element_tags = &mut element_tags as *mut *mut *mut usize;
        let element_tags_n = &mut element_tags_n as *mut *mut usize;
        let element_tags_nn = &mut element_tags_nn as *mut usize;
        let node_tags = &mut node_tags as *mut *mut *mut usize;
        let node_tags_n = &mut node_tags_n as *mut *mut usize;
        let node_tags_nn = &mut node_tags_nn as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetElements(
            element_types,
            element_types_n,
            element_tags,
            element_tags_n,
            element_tags_nn,
            node_tags,
            node_tags_n,
            node_tags_nn,
            dim,
            tag,
            ierr,
        );
    };

    if err == 0 {
        Ok((
            (element_types, element_types_n),
            (
                element_tags as *const *const usize,
                element_tags_n,
                element_tags_nn,
            ),
            (node_tags as *const *const usize, node_tags_n, node_tags_nn),
        ))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_element(
    element_tag: usize,
) -> GmshRawResult<(raw::c_int, (*const usize, usize), raw::c_int, raw::c_int)> {
    let mut element_type: raw::c_int = 0;
    let mut node_tags: *mut usize = std::ptr::null_mut();
    let mut node_tags_n: usize = 0;
    let mut dim: raw::c_int = 0;
    let mut tag: raw::c_int = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let element_type = &mut element_type as *mut raw::c_int;
        let node_tags = &mut node_tags as *mut *mut usize;
        let node_tags_n = &mut node_tags_n as *mut usize;
        let dim = &mut dim as *mut raw::c_int;
        let tag = &mut tag as *mut raw::c_int;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetElement(
            element_tag,
            element_type,
            node_tags,
            node_tags_n,
            dim,
            tag,
            ierr,
        );
    };

    if err == 0 {
        Ok((
            element_type,
            (node_tags as *const usize, node_tags_n),
            dim,
            tag,
        ))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_element_by_coordinates(
    x: f64,
    y: f64,
    z: f64,
    dim: raw::c_int,
    strict: raw::c_int,
) -> GmshRawResult<(usize, raw::c_int, (*const usize, usize), (f64, f64, f64))> {
    let mut element_tag: usize = 0;
    let mut element_type: raw::c_int = 0;
    let mut node_tags: *mut usize = std::ptr::null_mut();
    let mut node_tags_n: usize = 0;
    let mut u: f64 = 0.0;
    let mut v: f64 = 0.0;
    let mut w: f64 = 0.0;
    let mut err: raw::c_int = 0;

    unsafe {
        let element_tag = &mut element_tag as *mut usize;
        let element_type = &mut element_type as *mut raw::c_int;
        let node_tags = &mut node_tags as *mut *mut usize;
        let node_tags_n = &mut node_tags_n as *mut usize;
        let u = &mut u as *mut f64;
        let v = &mut v as *mut f64;
        let w = &mut w as *mut f64;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetElementByCoordinates(
            x,
            y,
            z,
            element_tag,
            element_type,
            node_tags,
            node_tags_n,
            u,
            v,
            w,
            dim,
            strict,
            ierr,
        );
    };

    if err == 0 {
        Ok((
            element_tag,
            element_type,
            (node_tags as *const usize, node_tags_n),
            (u, v, w),
        ))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_elements_by_coordinates(
    x: f64,
    y: f64,
    z: f64,
    dim: raw::c_int,
    strict: raw::c_int,
) -> GmshRawResult<(*const usize, usize)> {
    let mut element_tags: *mut usize = std::ptr::null_mut();
    let mut element_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let element_tags = &mut element_tags as *mut *mut usize;
        let element_tags_n = &mut element_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetElementsByCoordinates(
            x,
            y,
            z,
            element_tags,
            element_tags_n,
            dim,
            strict,
            ierr,
        );
    };

    if err == 0 {
        Ok((element_tags, element_tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_local_coordinates_in_element(
    element_tag: usize,
    x: f64,
    y: f64,
    z: f64,
) -> GmshRawResult<(f64, f64, f64)> {
    let mut u: f64 = 0.0;
    let mut v: f64 = 0.0;
    let mut w: f64 = 0.0;
    let mut err: raw::c_int = 0;

    unsafe {
        let u = &mut u as *mut f64;
        let v = &mut v as *mut f64;
        let w = &mut w as *mut f64;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetLocalCoordinatesInElement(element_tag, x, y, z, u, v, w, ierr);
    };

    if err == 0 {
        Ok((u, v, w))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_element_types(
    dim: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut element_types: *mut raw::c_int = std::ptr::null_mut();
    let mut element_types_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let element_types = &mut element_types as *mut *mut raw::c_int;
        let element_types_n = &mut element_types_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetElementTypes(element_types, element_types_n, dim, tag, ierr);
    };

    if err == 0 {
        Ok((element_types, element_types_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_element_type(
    family_name: *const raw::c_char,
    order: raw::c_int,
    serendip: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetElementType(family_name, order, serendip, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_element_properties(
    element_type: raw::c_int,
) -> GmshRawResult<(
    *mut raw::c_char,
    raw::c_int,
    raw::c_int,
    raw::c_int,
    (*const f64, usize),
    raw::c_int,
)> {
    let mut element_name: *mut raw::c_char = std::ptr::null_mut();
    let mut dim: raw::c_int = 0;
    let mut order: raw::c_int = 0;
    let mut num_nodes: raw::c_int = 0;
    let mut local_node_coord: *mut f64 = std::ptr::null_mut();
    let mut local_node_coord_n: usize = 0;
    let mut num_primary_nodes: raw::c_int = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let element_name = &mut element_name as *mut *mut raw::c_char;
        let dim = &mut dim as *mut raw::c_int;
        let order = &mut order as *mut raw::c_int;
        let num_nodes = &mut num_nodes as *mut raw::c_int;
        let local_node_coord = &mut local_node_coord as *mut *mut f64;
        let local_node_coord_n = &mut local_node_coord_n as *mut usize;
        let num_primary_nodes = &mut num_primary_nodes as *mut raw::c_int;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetElementProperties(
            element_type,
            element_name,
            dim,
            order,
            num_nodes,
            local_node_coord,
            local_node_coord_n,
            num_primary_nodes,
            ierr,
        );
    };

    if err == 0 {
        Ok((
            element_name,
            dim,
            order,
            num_nodes,
            (local_node_coord, local_node_coord_n),
            num_primary_nodes,
        ))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_elements_by_type(
    element_type: raw::c_int,
    tag: raw::c_int,
    task: usize,
    num_tasks: usize,
) -> GmshRawResult<((*const usize, usize), (*const usize, usize))> {
    let mut element_tags: *mut usize = std::ptr::null_mut();
    let mut element_tags_n: usize = 0;
    let mut node_tags: *mut usize = std::ptr::null_mut();
    let mut node_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let element_tags = &mut element_tags as *mut *mut usize;
        let element_tags_n = &mut element_tags_n as *mut usize;
        let node_tags = &mut node_tags as *mut *mut usize;
        let node_tags_n = &mut node_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetElementsByType(
            element_type,
            element_tags,
            element_tags_n,
            node_tags,
            node_tags_n,
            tag,
            task,
            num_tasks,
            ierr,
        );
    };

    if err == 0 {
        Ok(((element_tags, element_tags_n), (node_tags, node_tags_n)))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_max_element_tag() -> GmshRawResult<usize> {
    let mut max_tag: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let max_tag = &mut max_tag as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetMaxElementTag(max_tag, ierr);
    };

    if err == 0 {
        Ok(max_tag)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_preallocate_elements_by_type(
    element_type: raw::c_int,
    element_tag: raw::c_int,
    node_tag: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<((*const usize, usize), (*const usize, usize))> {
    let mut element_tags: *mut usize = std::ptr::null_mut();
    let mut element_tags_n: usize = 0;
    let mut node_tags: *mut usize = std::ptr::null_mut();
    let mut node_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let element_tags = &mut element_tags as *mut *mut usize;
        let element_tags_n = &mut element_tags_n as *mut usize;
        let node_tags = &mut node_tags as *mut *mut usize;
        let node_tags_n = &mut node_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshPreallocateElementsByType(
            element_type,
            element_tag,
            node_tag,
            element_tags,
            element_tags_n,
            node_tags,
            node_tags_n,
            tag,
            ierr,
        );
    };

    if err == 0 {
        Ok(((element_tags, element_tags_n), (node_tags, node_tags_n)))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_element_qualities(
    element_tags: *const usize,
    element_tags_n: usize,
    quality_name: *const raw::c_char,
    task: usize,
    num_tasks: usize,
) -> GmshRawResult<(*const f64, usize)> {
    let mut elements_quality: *mut f64 = std::ptr::null_mut();
    let mut elements_quality_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let elements_quality = &mut elements_quality as *mut *mut f64;
        let elements_quality_n = &mut elements_quality_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetElementQualities(
            element_tags,
            element_tags_n,
            elements_quality,
            elements_quality_n,
            quality_name,
            task,
            num_tasks,
            ierr,
        );
    };

    if err == 0 {
        Ok((elements_quality, elements_quality_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_add_elements(
    dim: raw::c_int,
    tag: raw::c_int,
    element_types: *const raw::c_int,
    element_types_n: usize,
    element_tags: *const *const usize,
    element_tags_n: *const usize,
    element_tags_nn: usize,
    node_tags: *const *const usize,
    node_tags_n: *const usize,
    node_tags_nn: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshAddElements(
            dim,
            tag,
            element_types,
            element_types_n,
            element_tags,
            element_tags_n,
            element_tags_nn,
            node_tags,
            node_tags_n,
            node_tags_nn,
            ierr,
        );
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_add_elements_by_type(
    tag: raw::c_int,
    element_type: raw::c_int,
    element_tags: *const usize,
    element_tags_n: usize,
    node_tags: *const usize,
    node_tags_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshAddElementsByType(
            tag,
            element_type,
            element_tags,
            element_tags_n,
            node_tags,
            node_tags_n,
            ierr,
        );
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_integration_points(
    element_type: raw::c_int,
    integration_type: *const raw::c_char,
) -> GmshRawResult<((*const f64, usize), (*const f64, usize))> {
    let mut local_coord: *mut f64 = std::ptr::null_mut();
    let mut local_coord_n: usize = 0;
    let mut weights: *mut f64 = std::ptr::null_mut();
    let mut weights_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let local_coord = &mut local_coord as *mut *mut f64;
        let local_coord_n = &mut local_coord_n as *mut usize;
        let weights = &mut weights as *mut *mut f64;
        let weights_n = &mut weights_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetIntegrationPoints(
            element_type,
            integration_type,
            local_coord,
            local_coord_n,
            weights,
            weights_n,
            ierr,
        );
    };

    if err == 0 {
        Ok(((local_coord, local_coord_n), (weights, weights_n)))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_jacobians(
    element_type: raw::c_int,
    local_coord: *const f64,
    local_coord_n: usize,
    tag: raw::c_int,
    task: usize,
    num_tasks: usize,
) -> GmshRawResult<(
    (*const f64, usize),
    (*const f64, usize),
    (*const f64, usize),
)> {
    let mut jacobians: *mut f64 = std::ptr::null_mut();
    let mut jacobians_n: usize = 0;
    let mut determinants: *mut f64 = std::ptr::null_mut();
    let mut determinants_n: usize = 0;
    let mut coord: *mut f64 = std::ptr::null_mut();
    let mut coord_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let jacobians = &mut jacobians as *mut *mut f64;
        let jacobians_n = &mut jacobians_n as *mut usize;
        let determinants = &mut determinants as *mut *mut f64;
        let determinants_n = &mut determinants_n as *mut usize;
        let coord = &mut coord as *mut *mut f64;
        let coord_n = &mut coord_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetJacobians(
            element_type,
            local_coord,
            local_coord_n,
            jacobians,
            jacobians_n,
            determinants,
            determinants_n,
            coord,
            coord_n,
            tag,
            task,
            num_tasks,
            ierr,
        );
    };

    if err == 0 {
        Ok((
            (jacobians, jacobians_n),
            (determinants, determinants_n),
            (coord, coord_n),
        ))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_preallocate_jacobians(
    element_type: raw::c_int,
    num_evaluation_points: raw::c_int,
    allocate_jacobians: raw::c_int,
    allocate_determinants: raw::c_int,
    allocate_coord: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<(
    (*const f64, usize),
    (*const f64, usize),
    (*const f64, usize),
)> {
    let mut jacobians: *mut f64 = std::ptr::null_mut();
    let mut jacobians_n: usize = 0;
    let mut determinants: *mut f64 = std::ptr::null_mut();
    let mut determinants_n: usize = 0;
    let mut coord: *mut f64 = std::ptr::null_mut();
    let mut coord_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let jacobians = &mut jacobians as *mut *mut f64;
        let jacobians_n = &mut jacobians_n as *mut usize;
        let determinants = &mut determinants as *mut *mut f64;
        let determinants_n = &mut determinants_n as *mut usize;
        let coord = &mut coord as *mut *mut f64;
        let coord_n = &mut coord_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshPreallocateJacobians(
            element_type,
            num_evaluation_points,
            allocate_jacobians,
            allocate_determinants,
            allocate_coord,
            jacobians,
            jacobians_n,
            determinants,
            determinants_n,
            coord,
            coord_n,
            tag,
            ierr,
        );
    };

    if err == 0 {
        Ok((
            (jacobians, jacobians_n),
            (determinants, determinants_n),
            (coord, coord_n),
        ))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_jacobian(
    element_tag: usize,
    local_coord: *const f64,
    local_coord_n: usize,
) -> GmshRawResult<(
    (*const f64, usize),
    (*const f64, usize),
    (*const f64, usize),
)> {
    let mut jacobians: *mut f64 = std::ptr::null_mut();
    let mut jacobians_n: usize = 0;
    let mut determinants: *mut f64 = std::ptr::null_mut();
    let mut determinants_n: usize = 0;
    let mut coord: *mut f64 = std::ptr::null_mut();
    let mut coord_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let jacobians = &mut jacobians as *mut *mut f64;
        let jacobians_n = &mut jacobians_n as *mut usize;
        let determinants = &mut determinants as *mut *mut f64;
        let determinants_n = &mut determinants_n as *mut usize;
        let coord = &mut coord as *mut *mut f64;
        let coord_n = &mut coord_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetJacobian(
            element_tag,
            local_coord,
            local_coord_n,
            jacobians,
            jacobians_n,
            determinants,
            determinants_n,
            coord,
            coord_n,
            ierr,
        );
    };

    if err == 0 {
        Ok((
            (jacobians, jacobians_n),
            (determinants, determinants_n),
            (coord, coord_n),
        ))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_basis_functions(
    element_type: raw::c_int,
    local_coord: *const f64,
    local_coord_n: usize,
    function_space_type: *const raw::c_char,
    wanted_orientations: *const raw::c_int,
    wanted_orientations_n: usize,
) -> GmshRawResult<(raw::c_int, (*const f64, usize), raw::c_int)> {
    let mut num_components: raw::c_int = 0;
    let mut basis_functions: *mut f64 = std::ptr::null_mut();
    let mut basis_functions_n: usize = 0;
    let mut num_orientations: raw::c_int = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let num_components = &mut num_components as *mut raw::c_int;
        let basis_functions = &mut basis_functions as *mut *mut f64;
        let basis_functions_n = &mut basis_functions_n as *mut usize;
        let num_orientations = &mut num_orientations as *mut raw::c_int;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetBasisFunctions(
            element_type,
            local_coord,
            local_coord_n,
            function_space_type,
            num_components,
            basis_functions,
            basis_functions_n,
            num_orientations,
            wanted_orientations,
            wanted_orientations_n,
            ierr,
        );
    };

    if err == 0 {
        Ok((
            num_components,
            (basis_functions, basis_functions_n),
            num_orientations,
        ))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_basis_functions_orientation(
    element_type: raw::c_int,
    function_space_type: *const raw::c_char,
    tag: raw::c_int,
    task: usize,
    num_tasks: usize,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut basis_functions_orientation: *mut raw::c_int = std::ptr::null_mut();
    let mut basis_functions_orientation_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let basis_functions_orientation = &mut basis_functions_orientation as *mut *mut raw::c_int;
        let basis_functions_orientation_n = &mut basis_functions_orientation_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetBasisFunctionsOrientation(
            element_type,
            function_space_type,
            basis_functions_orientation,
            basis_functions_orientation_n,
            tag,
            task,
            num_tasks,
            ierr,
        );
    };

    if err == 0 {
        Ok((basis_functions_orientation, basis_functions_orientation_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_basis_functions_orientation_for_element(
    element_tag: usize,
    function_space_type: *const raw::c_char,
) -> GmshRawResult<raw::c_int> {
    let mut basis_functions_orientation: raw::c_int = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let basis_functions_orientation = &mut basis_functions_orientation as *mut raw::c_int;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetBasisFunctionsOrientationForElement(
            element_tag,
            function_space_type,
            basis_functions_orientation,
            ierr,
        );
    };

    if err == 0 {
        Ok(basis_functions_orientation)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_number_of_orientations(
    element_type: raw::c_int,
    function_space_type: *const raw::c_char,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetNumberOfOrientations(element_type, function_space_type, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_preallocate_basis_functions_orientation(
    element_type: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut basis_functions_orientation: *mut raw::c_int = std::ptr::null_mut();
    let mut basis_functions_orientation_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let basis_functions_orientation = &mut basis_functions_orientation as *mut *mut raw::c_int;
        let basis_functions_orientation_n = &mut basis_functions_orientation_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshPreallocateBasisFunctionsOrientation(
            element_type,
            basis_functions_orientation,
            basis_functions_orientation_n,
            tag,
            ierr,
        );
    };

    if err == 0 {
        Ok((basis_functions_orientation, basis_functions_orientation_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_edges(
    node_tags: *const usize,
    node_tags_n: usize,
) -> GmshRawResult<((*const usize, usize), (*const raw::c_int, usize))> {
    let mut edge_tags: *mut usize = std::ptr::null_mut();
    let mut edge_tags_n: usize = 0;
    let mut edge_orientations: *mut raw::c_int = std::ptr::null_mut();
    let mut edge_orientations_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let edge_tags = &mut edge_tags as *mut *mut usize;
        let edge_tags_n = &mut edge_tags_n as *mut usize;
        let edge_orientations = &mut edge_orientations as *mut *mut raw::c_int;
        let edge_orientations_n = &mut edge_orientations_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetEdges(
            node_tags,
            node_tags_n,
            edge_tags,
            edge_tags_n,
            edge_orientations,
            edge_orientations_n,
            ierr,
        );
    };

    if err == 0 {
        Ok((
            (edge_tags, edge_tags_n),
            (edge_orientations, edge_orientations_n),
        ))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_faces(
    face_type: raw::c_int,
    node_tags: *const usize,
    node_tags_n: usize,
) -> GmshRawResult<((*const usize, usize), (*const raw::c_int, usize))> {
    let mut face_tags: *mut usize = std::ptr::null_mut();
    let mut face_tags_n: usize = 0;
    let mut face_orientations: *mut raw::c_int = std::ptr::null_mut();
    let mut face_orientations_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let face_tags = &mut face_tags as *mut *mut usize;
        let face_tags_n = &mut face_tags_n as *mut usize;
        let face_orientations = &mut face_orientations as *mut *mut raw::c_int;
        let face_orientations_n = &mut face_orientations_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetFaces(
            face_type,
            node_tags,
            node_tags_n,
            face_tags,
            face_tags_n,
            face_orientations,
            face_orientations_n,
            ierr,
        );
    };

    if err == 0 {
        Ok((
            (face_tags, face_tags_n),
            (face_orientations, face_orientations_n),
        ))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_create_edges(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshCreateEdges(dim_tags, dim_tags_n, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_create_faces(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshCreateFaces(dim_tags, dim_tags_n, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_all_edges()
-> GmshRawResult<((*const usize, usize), (*const usize, usize))> {
    let mut edge_tags: *mut usize = std::ptr::null_mut();
    let mut edge_tags_n: usize = 0;
    let mut edge_nodes: *mut usize = std::ptr::null_mut();
    let mut edge_nodes_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let edge_tags = &mut edge_tags as *mut *mut usize;
        let edge_tags_n = &mut edge_tags_n as *mut usize;
        let edge_nodes = &mut edge_nodes as *mut *mut usize;
        let edge_nodes_n = &mut edge_nodes_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetAllEdges(edge_tags, edge_tags_n, edge_nodes, edge_nodes_n, ierr);
    };

    if err == 0 {
        Ok(((edge_tags, edge_tags_n), (edge_nodes, edge_nodes_n)))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_all_faces(
    face_type: raw::c_int,
) -> GmshRawResult<((*const usize, usize), (*const usize, usize))> {
    let mut face_tags: *mut usize = std::ptr::null_mut();
    let mut face_tags_n: usize = 0;
    let mut face_nodes: *mut usize = std::ptr::null_mut();
    let mut face_nodes_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let face_tags = &mut face_tags as *mut *mut usize;
        let face_tags_n = &mut face_tags_n as *mut usize;
        let face_nodes = &mut face_nodes as *mut *mut usize;
        let face_nodes_n = &mut face_nodes_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetAllFaces(
            face_type,
            face_tags,
            face_tags_n,
            face_nodes,
            face_nodes_n,
            ierr,
        );
    };

    if err == 0 {
        Ok(((face_tags, face_tags_n), (face_nodes, face_nodes_n)))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_add_edges(
    edge_tags: *const usize,
    edge_tags_n: usize,
    edge_nodes: *const usize,
    edge_nodes_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshAddEdges(edge_tags, edge_tags_n, edge_nodes, edge_nodes_n, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_add_faces(
    face_type: raw::c_int,
    face_tags: *const usize,
    face_tags_n: usize,
    face_nodes: *const usize,
    face_nodes_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshAddFaces(
            face_type,
            face_tags,
            face_tags_n,
            face_nodes,
            face_nodes_n,
            ierr,
        );
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_keys(
    element_type: raw::c_int,
    function_space_type: *const raw::c_char,
    tag: raw::c_int,
    return_coord: raw::c_int,
) -> GmshRawResult<(
    (*const raw::c_int, usize),
    (*const usize, usize),
    (*const f64, usize),
)> {
    let mut type_keys: *mut raw::c_int = std::ptr::null_mut();
    let mut type_keys_n: usize = 0;
    let mut entity_keys: *mut usize = std::ptr::null_mut();
    let mut entity_keys_n: usize = 0;
    let mut coord: *mut f64 = std::ptr::null_mut();
    let mut coord_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let type_keys = &mut type_keys as *mut *mut raw::c_int;
        let type_keys_n = &mut type_keys_n as *mut usize;
        let entity_keys = &mut entity_keys as *mut *mut usize;
        let entity_keys_n = &mut entity_keys_n as *mut usize;
        let coord = &mut coord as *mut *mut f64;
        let coord_n = &mut coord_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetKeys(
            element_type,
            function_space_type,
            type_keys,
            type_keys_n,
            entity_keys,
            entity_keys_n,
            coord,
            coord_n,
            tag,
            return_coord,
            ierr,
        );
    };

    if err == 0 {
        Ok((
            (type_keys, type_keys_n),
            (entity_keys, entity_keys_n),
            (coord, coord_n),
        ))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_keys_for_element(
    element_tag: usize,
    function_space_type: *const raw::c_char,
    return_coord: raw::c_int,
) -> GmshRawResult<(
    (*const raw::c_int, usize),
    (*const usize, usize),
    (*const f64, usize),
)> {
    let mut type_keys: *mut raw::c_int = std::ptr::null_mut();
    let mut type_keys_n: usize = 0;
    let mut entity_keys: *mut usize = std::ptr::null_mut();
    let mut entity_keys_n: usize = 0;
    let mut coord: *mut f64 = std::ptr::null_mut();
    let mut coord_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let type_keys = &mut type_keys as *mut *mut raw::c_int;
        let type_keys_n = &mut type_keys_n as *mut usize;
        let entity_keys = &mut entity_keys as *mut *mut usize;
        let entity_keys_n = &mut entity_keys_n as *mut usize;
        let coord = &mut coord as *mut *mut f64;
        let coord_n = &mut coord_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetKeysForElement(
            element_tag,
            function_space_type,
            type_keys,
            type_keys_n,
            entity_keys,
            entity_keys_n,
            coord,
            coord_n,
            return_coord,
            ierr,
        );
    };

    if err == 0 {
        Ok((
            (type_keys, type_keys_n),
            (entity_keys, entity_keys_n),
            (coord, coord_n),
        ))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_number_of_keys(
    element_type: raw::c_int,
    function_space_type: *const raw::c_char,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetNumberOfKeys(element_type, function_space_type, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_keys_information(
    type_keys: *const raw::c_int,
    type_keys_n: usize,
    entity_keys: *const usize,
    entity_keys_n: usize,
    element_type: raw::c_int,
    function_space_type: *const raw::c_char,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut info_keys: *mut raw::c_int = std::ptr::null_mut();
    let mut info_keys_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let info_keys = &mut info_keys as *mut *mut raw::c_int;
        let info_keys_n = &mut info_keys_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetKeysInformation(
            type_keys,
            type_keys_n,
            entity_keys,
            entity_keys_n,
            element_type,
            function_space_type,
            info_keys,
            info_keys_n,
            ierr,
        );
    };

    if err == 0 {
        Ok((info_keys, info_keys_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_barycenters(
    element_type: raw::c_int,
    tag: raw::c_int,
    fast: raw::c_int,
    primary: raw::c_int,
    task: usize,
    num_tasks: usize,
) -> GmshRawResult<(*const f64, usize)> {
    let mut barycenters: *mut f64 = std::ptr::null_mut();
    let mut barycenters_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let barycenters = &mut barycenters as *mut *mut f64;
        let barycenters_n = &mut barycenters_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetBarycenters(
            element_type,
            tag,
            fast,
            primary,
            barycenters,
            barycenters_n,
            task,
            num_tasks,
            ierr,
        );
    };

    if err == 0 {
        Ok((barycenters, barycenters_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_preallocate_barycenters(
    element_type: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<(*const f64, usize)> {
    let mut barycenters: *mut f64 = std::ptr::null_mut();
    let mut barycenters_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let barycenters = &mut barycenters as *mut *mut f64;
        let barycenters_n = &mut barycenters_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshPreallocateBarycenters(
            element_type,
            barycenters,
            barycenters_n,
            tag,
            ierr,
        );
    };

    if err == 0 {
        Ok((barycenters, barycenters_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_element_edge_nodes(
    element_type: raw::c_int,
    tag: raw::c_int,
    primary: raw::c_int,
    task: usize,
    num_tasks: usize,
) -> GmshRawResult<(*const usize, usize)> {
    let mut node_tags: *mut usize = std::ptr::null_mut();
    let mut node_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let node_tags = &mut node_tags as *mut *mut usize;
        let node_tags_n = &mut node_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetElementEdgeNodes(
            element_type,
            node_tags,
            node_tags_n,
            tag,
            primary,
            task,
            num_tasks,
            ierr,
        );
    };

    if err == 0 {
        Ok((node_tags, node_tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_element_face_nodes(
    element_type: raw::c_int,
    face_type: raw::c_int,
    tag: raw::c_int,
    primary: raw::c_int,
    task: usize,
    num_tasks: usize,
) -> GmshRawResult<(*const usize, usize)> {
    let mut node_tags: *mut usize = std::ptr::null_mut();
    let mut node_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let node_tags = &mut node_tags as *mut *mut usize;
        let node_tags_n = &mut node_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetElementFaceNodes(
            element_type,
            face_type,
            node_tags,
            node_tags_n,
            tag,
            primary,
            task,
            num_tasks,
            ierr,
        );
    };

    if err == 0 {
        Ok((node_tags, node_tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_ghost_elements(
    dim: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<((*const usize, usize), (*const raw::c_int, usize))> {
    let mut element_tags: *mut usize = std::ptr::null_mut();
    let mut element_tags_n: usize = 0;
    let mut partitions: *mut raw::c_int = std::ptr::null_mut();
    let mut partitions_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let element_tags = &mut element_tags as *mut *mut usize;
        let element_tags_n = &mut element_tags_n as *mut usize;
        let partitions = &mut partitions as *mut *mut raw::c_int;
        let partitions_n = &mut partitions_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetGhostElements(
            dim,
            tag,
            element_tags,
            element_tags_n,
            partitions,
            partitions_n,
            ierr,
        );
    };

    if err == 0 {
        Ok(((element_tags, element_tags_n), (partitions, partitions_n)))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_set_size(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    size: f64,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshSetSize(dim_tags, dim_tags_n, size, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_sizes(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
) -> GmshRawResult<(*const f64, usize)> {
    let mut sizes: *mut f64 = std::ptr::null_mut();
    let mut sizes_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let sizes = &mut sizes as *mut *mut f64;
        let sizes_n = &mut sizes_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetSizes(dim_tags, dim_tags_n, sizes, sizes_n, ierr);
    };

    if err == 0 {
        Ok((sizes, sizes_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_set_size_at_parametric_points(
    dim: raw::c_int,
    tag: raw::c_int,
    parametric_coord: *const f64,
    parametric_coord_n: usize,
    sizes: *const f64,
    sizes_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshSetSizeAtParametricPoints(
            dim,
            tag,
            parametric_coord,
            parametric_coord_n,
            sizes,
            sizes_n,
            ierr,
        );
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_set_size_callback(
    callback: ::std::option::Option<
        unsafe extern "C" fn(
            dim: raw::c_int,
            tag: raw::c_int,
            x: f64,
            y: f64,
            z: f64,
            lc: f64,
            data: *mut raw::c_void,
        ) -> f64,
    >,
    callback_data: *mut raw::c_void,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshSetSizeCallback(callback, callback_data, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_remove_size_callback() -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshRemoveSizeCallback(ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_set_transfinite_curve(
    tag: raw::c_int,
    num_nodes: raw::c_int,
    mesh_type: *const raw::c_char,
    coef: f64,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshSetTransfiniteCurve(tag, num_nodes, mesh_type, coef, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_set_transfinite_surface(
    tag: raw::c_int,
    arrangement: *const raw::c_char,
    corner_tags: *const raw::c_int,
    corner_tags_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshSetTransfiniteSurface(tag, arrangement, corner_tags, corner_tags_n, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_set_transfinite_volume(
    tag: raw::c_int,
    corner_tags: *const raw::c_int,
    corner_tags_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshSetTransfiniteVolume(tag, corner_tags, corner_tags_n, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_set_transfinite_automatic(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    corner_angle: f64,
    recombine: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshSetTransfiniteAutomatic(
            dim_tags,
            dim_tags_n,
            corner_angle,
            recombine,
            ierr,
        );
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_set_recombine(
    dim: raw::c_int,
    tag: raw::c_int,
    angle: f64,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshSetRecombine(dim, tag, angle, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_set_smoothing(
    dim: raw::c_int,
    tag: raw::c_int,
    val: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshSetSmoothing(dim, tag, val, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_set_reverse(
    dim: raw::c_int,
    tag: raw::c_int,
    val: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshSetReverse(dim, tag, val, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_set_algorithm(
    dim: raw::c_int,
    tag: raw::c_int,
    val: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshSetAlgorithm(dim, tag, val, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_set_size_from_boundary(
    dim: raw::c_int,
    tag: raw::c_int,
    val: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshSetSizeFromBoundary(dim, tag, val, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_set_compound(
    dim: raw::c_int,
    tags: *const raw::c_int,
    tags_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshSetCompound(dim, tags, tags_n, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_set_outward_orientation(tag: raw::c_int) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshSetOutwardOrientation(tag, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_remove_constraints(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshRemoveConstraints(dim_tags, dim_tags_n, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_embed(
    dim: raw::c_int,
    tags: *const raw::c_int,
    tags_n: usize,
    in_dim: raw::c_int,
    in_tag: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshEmbed(dim, tags, tags_n, in_dim, in_tag, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_remove_embedded(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    dim: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshRemoveEmbedded(dim_tags, dim_tags_n, dim, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_embedded(
    dim: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut dim_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut dim_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let dim_tags = &mut dim_tags as *mut *mut raw::c_int;
        let dim_tags_n = &mut dim_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetEmbedded(dim, tag, dim_tags, dim_tags_n, ierr);
    };

    if err == 0 {
        Ok((dim_tags, dim_tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_reorder_elements(
    element_type: raw::c_int,
    tag: raw::c_int,
    ordering: *const usize,
    ordering_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshReorderElements(element_type, tag, ordering, ordering_n, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_compute_renumbering(
    method: *const raw::c_char,
    element_tags: *const usize,
    element_tags_n: usize,
) -> GmshRawResult<((*const usize, usize), (*const usize, usize))> {
    let mut old_tags: *mut usize = std::ptr::null_mut();
    let mut old_tags_n: usize = 0;
    let mut new_tags: *mut usize = std::ptr::null_mut();
    let mut new_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let old_tags = &mut old_tags as *mut *mut usize;
        let old_tags_n = &mut old_tags_n as *mut usize;
        let new_tags = &mut new_tags as *mut *mut usize;
        let new_tags_n = &mut new_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshComputeRenumbering(
            old_tags,
            old_tags_n,
            new_tags,
            new_tags_n,
            method,
            element_tags,
            element_tags_n,
            ierr,
        );
    };

    if err == 0 {
        Ok(((old_tags, old_tags_n), (new_tags, new_tags_n)))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_renumber_nodes(
    old_tags: *const usize,
    old_tags_n: usize,
    new_tags: *const usize,
    new_tags_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshRenumberNodes(old_tags, old_tags_n, new_tags, new_tags_n, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_renumber_elements(
    old_tags: *const usize,
    old_tags_n: usize,
    new_tags: *const usize,
    new_tags_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshRenumberElements(old_tags, old_tags_n, new_tags, new_tags_n, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_set_periodic(
    dim: raw::c_int,
    tags: *const raw::c_int,
    tags_n: usize,
    tags_master: *const raw::c_int,
    tags_master_n: usize,
    affine_transform: *const f64,
    affine_transform_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshSetPeriodic(
            dim,
            tags,
            tags_n,
            tags_master,
            tags_master_n,
            affine_transform,
            affine_transform_n,
            ierr,
        );
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_periodic(
    dim: raw::c_int,
    tags: *const raw::c_int,
    tags_n: usize,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut tag_master: *mut raw::c_int = std::ptr::null_mut();
    let mut tag_master_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let tag_master = &mut tag_master as *mut *mut raw::c_int;
        let tag_master_n = &mut tag_master_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetPeriodic(dim, tags, tags_n, tag_master, tag_master_n, ierr);
    };

    if err == 0 {
        Ok((tag_master, tag_master_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_periodic_nodes(
    dim: raw::c_int,
    tag: raw::c_int,
    include_high_order_nodes: raw::c_int,
) -> GmshRawResult<(
    raw::c_int,
    (*const usize, usize),
    (*const usize, usize),
    (*const f64, usize),
)> {
    let mut tag_master: raw::c_int = 0;
    let mut node_tags: *mut usize = std::ptr::null_mut();
    let mut node_tags_n: usize = 0;
    let mut node_tags_master: *mut usize = std::ptr::null_mut();
    let mut node_tags_master_n: usize = 0;
    let mut affine_transform: *mut f64 = std::ptr::null_mut();
    let mut affine_transform_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let tag_master = &mut tag_master as *mut raw::c_int;
        let node_tags = &mut node_tags as *mut *mut usize;
        let node_tags_n = &mut node_tags_n as *mut usize;
        let node_tags_master = &mut node_tags_master as *mut *mut usize;
        let node_tags_master_n = &mut node_tags_master_n as *mut usize;
        let affine_transform = &mut affine_transform as *mut *mut f64;
        let affine_transform_n = &mut affine_transform_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetPeriodicNodes(
            dim,
            tag,
            tag_master,
            node_tags,
            node_tags_n,
            node_tags_master,
            node_tags_master_n,
            affine_transform,
            affine_transform_n,
            include_high_order_nodes,
            ierr,
        );
    };

    if err == 0 {
        Ok((
            tag_master,
            (node_tags, node_tags_n),
            (node_tags_master, node_tags_master_n),
            (affine_transform, affine_transform_n),
        ))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_periodic_keys(
    element_type: raw::c_int,
    function_space_type: *const raw::c_char,
    tag: raw::c_int,
    return_coord: raw::c_int,
) -> GmshRawResult<(
    raw::c_int,
    (*const raw::c_int, usize),
    (*const raw::c_int, usize),
    (*const usize, usize),
    (*const usize, usize),
    (*const f64, usize),
    (*const f64, usize),
)> {
    let mut tag_master: raw::c_int = 0;
    let mut type_keys: *mut raw::c_int = std::ptr::null_mut();
    let mut type_keys_n: usize = 0;
    let mut type_keys_master: *mut raw::c_int = std::ptr::null_mut();
    let mut type_keys_master_n: usize = 0;
    let mut entity_keys: *mut usize = std::ptr::null_mut();
    let mut entity_keys_n: usize = 0;
    let mut entity_keys_master: *mut usize = std::ptr::null_mut();
    let mut entity_keys_master_n: usize = 0;
    let mut coord: *mut f64 = std::ptr::null_mut();
    let mut coord_n: usize = 0;
    let mut coord_master: *mut f64 = std::ptr::null_mut();
    let mut coord_master_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let tag_master = &mut tag_master as *mut raw::c_int;
        let type_keys = &mut type_keys as *mut *mut raw::c_int;
        let type_keys_n = &mut type_keys_n as *mut usize;
        let type_keys_master = &mut type_keys_master as *mut *mut raw::c_int;
        let type_keys_master_n = &mut type_keys_master_n as *mut usize;
        let entity_keys = &mut entity_keys as *mut *mut usize;
        let entity_keys_n = &mut entity_keys_n as *mut usize;
        let entity_keys_master = &mut entity_keys_master as *mut *mut usize;
        let entity_keys_master_n = &mut entity_keys_master_n as *mut usize;
        let coord = &mut coord as *mut *mut f64;
        let coord_n = &mut coord_n as *mut usize;
        let coord_master = &mut coord_master as *mut *mut f64;
        let coord_master_n = &mut coord_master_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetPeriodicKeys(
            element_type,
            function_space_type,
            tag,
            tag_master,
            type_keys,
            type_keys_n,
            type_keys_master,
            type_keys_master_n,
            entity_keys,
            entity_keys_n,
            entity_keys_master,
            entity_keys_master_n,
            coord,
            coord_n,
            coord_master,
            coord_master_n,
            return_coord,
            ierr,
        );
    };

    if err == 0 {
        Ok((
            tag_master,
            (type_keys, type_keys_n),
            (type_keys_master, type_keys_master_n),
            (entity_keys, entity_keys_n),
            (entity_keys_master, entity_keys_master_n),
            (coord, coord_n),
            (coord_master, coord_master_n),
        ))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_import_stl() -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshImportStl(ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_duplicate_nodes(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
) -> GmshRawResult<(*const usize, usize)> {
    let mut tags: *mut usize = std::ptr::null_mut();
    let mut tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let tags = &mut tags as *mut *mut usize;
        let tags_n = &mut tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetDuplicateNodes(tags, tags_n, dim_tags, dim_tags_n, ierr);
    };

    if err == 0 {
        Ok((tags, tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_remove_duplicate_nodes(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshRemoveDuplicateNodes(dim_tags, dim_tags_n, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_remove_duplicate_elements(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshRemoveDuplicateElements(dim_tags, dim_tags_n, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_split_quadrangles(
    quality: f64,
    tag: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshSplitQuadrangles(quality, tag, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_set_visibility(
    element_tags: *const usize,
    element_tags_n: usize,
    value: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshSetVisibility(element_tags, element_tags_n, value, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_get_visibility(
    element_tags: *const usize,
    element_tags_n: usize,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut values: *mut raw::c_int = std::ptr::null_mut();
    let mut values_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let values = &mut values as *mut *mut raw::c_int;
        let values_n = &mut values_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshGetVisibility(element_tags, element_tags_n, values, values_n, ierr);
    };

    if err == 0 {
        Ok((values, values_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_classify_surfaces(
    angle: f64,
    boundary: raw::c_int,
    for_reparametrization: raw::c_int,
    curve_angle: f64,
    export_discrete: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshClassifySurfaces(
            angle,
            boundary,
            for_reparametrization,
            curve_angle,
            export_discrete,
            ierr,
        );
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_create_geometry(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshCreateGeometry(dim_tags, dim_tags_n, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_create_topology(
    make_simply_connected: raw::c_int,
    export_discrete: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshCreateTopology(make_simply_connected, export_discrete, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_add_homology_request(
    type_: *const raw::c_char,
    domain_tags: *const raw::c_int,
    domain_tags_n: usize,
    subdomain_tags: *const raw::c_int,
    subdomain_tags_n: usize,
    dims: *const raw::c_int,
    dims_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshAddHomologyRequest(
            type_,
            domain_tags,
            domain_tags_n,
            subdomain_tags,
            subdomain_tags_n,
            dims,
            dims_n,
            ierr,
        );
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_clear_homology_requests() -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshClearHomologyRequests(ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_compute_homology() -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut dim_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut dim_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let dim_tags = &mut dim_tags as *mut *mut raw::c_int;
        let dim_tags_n = &mut dim_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshComputeHomology(dim_tags, dim_tags_n, ierr);
    };

    if err == 0 {
        Ok((dim_tags, dim_tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_compute_cross_field() -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut view_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut view_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let view_tags = &mut view_tags as *mut *mut raw::c_int;
        let view_tags_n = &mut view_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshComputeCrossField(view_tags, view_tags_n, ierr);
    };

    if err == 0 {
        Ok((view_tags, view_tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_triangulate(
    coord: *const f64,
    coord_n: usize,
) -> GmshRawResult<(*const usize, usize)> {
    let mut tri: *mut usize = std::ptr::null_mut();
    let mut tri_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let tri = &mut tri as *mut *mut usize;
        let tri_n = &mut tri_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshTriangulate(coord, coord_n, tri, tri_n, ierr);
    };

    if err == 0 {
        Ok((tri, tri_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_tetrahedralize(
    coord: *const f64,
    coord_n: usize,
) -> GmshRawResult<(*const usize, usize)> {
    let mut tetra: *mut usize = std::ptr::null_mut();
    let mut tetra_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let tetra = &mut tetra as *mut *mut usize;
        let tetra_n = &mut tetra_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshTetrahedralize(coord, coord_n, tetra, tetra_n, ierr);
    };

    if err == 0 {
        Ok((tetra, tetra_n))
    } else {
        Err(GmshRawError(err))
    }
}

/* gmsh/model/mesh/field namespace */

pub unsafe fn gmsh_model_mesh_field_add(
    field_type: *const raw::c_char,
    tag: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshFieldAdd(field_type, tag, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_field_remove(tag: raw::c_int) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshFieldRemove(tag, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_field_list() -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut tags: *mut raw::c_int = std::ptr::null_mut();
    let mut tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let tags = &mut tags as *mut *mut raw::c_int;
        let tags_n = &mut tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshFieldList(tags, tags_n, ierr);
    };

    if err == 0 {
        Ok((tags, tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_field_get_type(tag: raw::c_int) -> GmshRawResult<*mut raw::c_char> {
    let mut file_type: *mut raw::c_char = std::ptr::null_mut();
    let mut err: raw::c_int = 0;

    unsafe {
        let file_type = &mut file_type as *mut *mut raw::c_char;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshFieldGetType(tag, file_type, ierr);
    };

    if err == 0 {
        Ok(file_type)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_field_set_number(
    tag: raw::c_int,
    option: *const raw::c_char,
    value: f64,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshFieldSetNumber(tag, option, value, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_field_get_number(
    tag: raw::c_int,
    option: *const raw::c_char,
) -> GmshRawResult<f64> {
    let mut value: f64 = 0.0;
    let mut err: raw::c_int = 0;

    unsafe {
        let value = &mut value as *mut f64;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshFieldGetNumber(tag, option, value, ierr);
    };

    if err == 0 {
        Ok(value)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_field_set_string(
    tag: raw::c_int,
    option: *const raw::c_char,
    value: *const raw::c_char,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshFieldSetString(tag, option, value, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_field_get_string(
    tag: raw::c_int,
    option: *const raw::c_char,
) -> GmshRawResult<*mut raw::c_char> {
    let mut value: *mut raw::c_char = std::ptr::null_mut();
    let mut err: raw::c_int = 0;

    unsafe {
        let value = &mut value as *mut *mut raw::c_char;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshFieldGetString(tag, option, value, ierr);
    };

    if err == 0 {
        Ok(value)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_field_set_numbers(
    tag: raw::c_int,
    option: *const raw::c_char,
    values: *const f64,
    values_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshFieldSetNumbers(tag, option, values, values_n, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_field_get_numbers(
    tag: raw::c_int,
    option: *const raw::c_char,
) -> GmshRawResult<(*const f64, usize)> {
    let mut values: *mut f64 = std::ptr::null_mut();
    let mut values_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let values = &mut values as *mut *mut f64;
        let values_n = &mut values_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshFieldGetNumbers(tag, option, values, values_n, ierr);
    };

    if err == 0 {
        Ok((values, values_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_field_set_as_background_mesh(tag: raw::c_int) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshFieldSetAsBackgroundMesh(tag, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_mesh_field_set_as_boundary_layer(tag: raw::c_int) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelMeshFieldSetAsBoundaryLayer(tag, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

/* gmsh/model/geo namespace */

pub unsafe fn gmsh_model_geo_add_point(
    x: f64,
    y: f64,
    z: f64,
    mesh_size: f64,
    tag: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoAddPoint(x, y, z, mesh_size, tag, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_add_line(
    start_tag: raw::c_int,
    end_tag: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoAddLine(start_tag, end_tag, tag, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_add_circle_arc(
    start_tag: raw::c_int,
    center_tag: raw::c_int,
    end_tag: raw::c_int,
    tag: raw::c_int,
    nx: f64,
    ny: f64,
    nz: f64,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoAddCircleArc(start_tag, center_tag, end_tag, tag, nx, ny, nz, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_add_ellipse_arc(
    start_tag: raw::c_int,
    center_tag: raw::c_int,
    major_tag: raw::c_int,
    end_tag: raw::c_int,
    tag: raw::c_int,
    nx: f64,
    ny: f64,
    nz: f64,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoAddEllipseArc(
            start_tag, center_tag, major_tag, end_tag, tag, nx, ny, nz, ierr,
        )
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_add_spline(
    point_tags: *const raw::c_int,
    point_tags_n: usize,
    tag: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoAddSpline(point_tags, point_tags_n, tag, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_add_bspline(
    point_tags: *const raw::c_int,
    point_tags_n: usize,
    tag: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoAddBSpline(point_tags, point_tags_n, tag, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_add_bezier(
    point_tags: *const raw::c_int,
    point_tags_n: usize,
    tag: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoAddBezier(point_tags, point_tags_n, tag, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_add_polyline(
    point_tags: *const raw::c_int,
    point_tags_n: usize,
    tag: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoAddPolyline(point_tags, point_tags_n, tag, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_add_compound_spline(
    curve_tags: *const raw::c_int,
    curve_tags_n: usize,
    num_intervals: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoAddCompoundSpline(curve_tags, curve_tags_n, num_intervals, tag, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_add_compound_bspline(
    curve_tags: *const raw::c_int,
    curve_tags_n: usize,
    num_intervals: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoAddCompoundBSpline(curve_tags, curve_tags_n, num_intervals, tag, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_add_curve_loop(
    curve_tags: *const raw::c_int,
    curve_tags_n: usize,
    tag: raw::c_int,
    reorient: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoAddCurveLoop(curve_tags, curve_tags_n, tag, reorient, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_add_curve_loops(
    curve_tags: *const raw::c_int,
    curve_tags_n: usize,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut tags: *mut raw::c_int = std::ptr::null_mut();
    let mut tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let tags = &mut tags as *mut *mut raw::c_int;
        let tags_n = &mut tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoAddCurveLoops(curve_tags, curve_tags_n, tags, tags_n, ierr);
    };

    if err == 0 {
        Ok((tags, tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_add_plane_surface(
    wire_tags: *const raw::c_int,
    wire_tags_n: usize,
    tag: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoAddPlaneSurface(wire_tags, wire_tags_n, tag, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_add_surface_filling(
    wire_tags: *const raw::c_int,
    wire_tags_n: usize,
    tag: raw::c_int,
    sphere_center_tag: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoAddSurfaceFilling(wire_tags, wire_tags_n, tag, sphere_center_tag, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_add_surface_loop(
    surface_tags: *const raw::c_int,
    surface_tags_n: usize,
    tag: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoAddSurfaceLoop(surface_tags, surface_tags_n, tag, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_add_volume(
    shell_tags: *const raw::c_int,
    shell_tags_n: usize,
    tag: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoAddVolume(shell_tags, shell_tags_n, tag, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_add_geometry(
    geometry: *const raw::c_char,
    numbers: *const f64,
    numbers_n: usize,
    strings: *const *const raw::c_char,
    strings_n: usize,
    tag: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoAddGeometry(geometry, numbers, numbers_n, strings, strings_n, tag, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_add_point_on_geometry(
    geometry_tag: raw::c_int,
    x: f64,
    y: f64,
    z: f64,
    mesh_size: f64,
    tag: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoAddPointOnGeometry(geometry_tag, x, y, z, mesh_size, tag, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_extrude(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    dx: f64,
    dy: f64,
    dz: f64,
    num_elements: *const raw::c_int,
    num_elements_n: usize,
    heights: *const f64,
    heights_n: usize,
    recombine: raw::c_int,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut out_dim_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut out_dim_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let out_dim_tags = &mut out_dim_tags as *mut *mut raw::c_int;
        let out_dim_tags_n = &mut out_dim_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoExtrude(
            dim_tags,
            dim_tags_n,
            dx,
            dy,
            dz,
            out_dim_tags,
            out_dim_tags_n,
            num_elements,
            num_elements_n,
            heights,
            heights_n,
            recombine,
            ierr,
        );
    };

    if err == 0 {
        Ok((out_dim_tags, out_dim_tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_revolve(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    x: f64,
    y: f64,
    z: f64,
    ax: f64,
    ay: f64,
    az: f64,
    angle: f64,
    num_elements: *const raw::c_int,
    num_elements_n: usize,
    heights: *const f64,
    heights_n: usize,
    recombine: raw::c_int,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut out_dim_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut out_dim_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let out_dim_tags = &mut out_dim_tags as *mut *mut raw::c_int;
        let out_dim_tags_n = &mut out_dim_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoRevolve(
            dim_tags,
            dim_tags_n,
            x,
            y,
            z,
            ax,
            ay,
            az,
            angle,
            out_dim_tags,
            out_dim_tags_n,
            num_elements,
            num_elements_n,
            heights,
            heights_n,
            recombine,
            ierr,
        );
    };

    if err == 0 {
        Ok((out_dim_tags, out_dim_tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_twist(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    x: f64,
    y: f64,
    z: f64,
    dx: f64,
    dy: f64,
    dz: f64,
    ax: f64,
    ay: f64,
    az: f64,
    angle: f64,
    num_elements: *const raw::c_int,
    num_elements_n: usize,
    heights: *const f64,
    heights_n: usize,
    recombine: raw::c_int,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut out_dim_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut out_dim_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let out_dim_tags = &mut out_dim_tags as *mut *mut raw::c_int;
        let out_dim_tags_n = &mut out_dim_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoTwist(
            dim_tags,
            dim_tags_n,
            x,
            y,
            z,
            dx,
            dy,
            dz,
            ax,
            ay,
            az,
            angle,
            out_dim_tags,
            out_dim_tags_n,
            num_elements,
            num_elements_n,
            heights,
            heights_n,
            recombine,
            ierr,
        );
    };

    if err == 0 {
        Ok((out_dim_tags, out_dim_tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_extrude_boundary_layer(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    num_elements: *const raw::c_int,
    num_elements_n: usize,
    heights: *const f64,
    heights_n: usize,
    recombine: raw::c_int,
    second: raw::c_int,
    view_index: raw::c_int,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut out_dim_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut out_dim_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let out_dim_tags = &mut out_dim_tags as *mut *mut raw::c_int;
        let out_dim_tags_n = &mut out_dim_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoExtrudeBoundaryLayer(
            dim_tags,
            dim_tags_n,
            out_dim_tags,
            out_dim_tags_n,
            num_elements,
            num_elements_n,
            heights,
            heights_n,
            recombine,
            second,
            view_index,
            ierr,
        );
    };

    if err == 0 {
        Ok((out_dim_tags, out_dim_tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_translate(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    dx: f64,
    dy: f64,
    dz: f64,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoTranslate(dim_tags, dim_tags_n, dx, dy, dz, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_rotate(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    x: f64,
    y: f64,
    z: f64,
    ax: f64,
    ay: f64,
    az: f64,
    angle: f64,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoRotate(dim_tags, dim_tags_n, x, y, z, ax, ay, az, angle, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_dilate(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    x: f64,
    y: f64,
    z: f64,
    a: f64,
    b: f64,
    c: f64,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoDilate(dim_tags, dim_tags_n, x, y, z, a, b, c, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_mirror(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoMirror(dim_tags, dim_tags_n, a, b, c, d, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_symmetrize(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoSymmetrize(dim_tags, dim_tags_n, a, b, c, d, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_copy(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut out_dim_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut out_dim_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let out_dim_tags = &mut out_dim_tags as *mut *mut raw::c_int;
        let out_dim_tags_n = &mut out_dim_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoCopy(dim_tags, dim_tags_n, out_dim_tags, out_dim_tags_n, ierr);
    };

    if err == 0 {
        Ok((out_dim_tags, out_dim_tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_remove(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    recursive: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoRemove(dim_tags, dim_tags_n, recursive, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_remove_all_duplicates() -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoRemoveAllDuplicates(ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_split_curve(
    tag: raw::c_int,
    point_tags: *const raw::c_int,
    point_tags_n: usize,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut curve_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut curve_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let curve_tags = &mut curve_tags as *mut *mut raw::c_int;
        let curve_tags_n = &mut curve_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoSplitCurve(
            tag,
            point_tags,
            point_tags_n,
            curve_tags,
            curve_tags_n,
            ierr,
        );
    };

    if err == 0 {
        Ok((curve_tags, curve_tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_get_max_tag(dim: raw::c_int) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoGetMaxTag(dim, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_set_max_tag(
    dim: raw::c_int,
    max_tag: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoSetMaxTag(dim, max_tag, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_add_physical_group(
    dim: raw::c_int,
    tags: *const raw::c_int,
    tags_n: usize,
    tag: raw::c_int,
    name: *const raw::c_char,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoAddPhysicalGroup(dim, tags, tags_n, tag, name, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_remove_physical_groups(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoRemovePhysicalGroups(dim_tags, dim_tags_n, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_synchronize() -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoSynchronize(ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

/* gmsh/model/geo/mesh namespace */

pub unsafe fn gmsh_model_geo_mesh_set_size(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    size: f64,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoMeshSetSize(dim_tags, dim_tags_n, size, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_mesh_set_transfinite_curve(
    tag: raw::c_int,
    n_points: raw::c_int,
    mesh_type: *const raw::c_char,
    coef: f64,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoMeshSetTransfiniteCurve(tag, n_points, mesh_type, coef, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_mesh_set_transfinite_surface(
    tag: raw::c_int,
    arrangement: *const raw::c_char,
    corner_tags: *const raw::c_int,
    corner_tags_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoMeshSetTransfiniteSurface(
            tag,
            arrangement,
            corner_tags,
            corner_tags_n,
            ierr,
        );
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_mesh_set_transfinite_volume(
    tag: raw::c_int,
    corner_tags: *const raw::c_int,
    corner_tags_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoMeshSetTransfiniteVolume(tag, corner_tags, corner_tags_n, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_mesh_set_recombine(
    dim: raw::c_int,
    tag: raw::c_int,
    angle: f64,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoMeshSetRecombine(dim, tag, angle, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_mesh_set_smoothing(
    dim: raw::c_int,
    tag: raw::c_int,
    val: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoMeshSetSmoothing(dim, tag, val, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_mesh_set_reverse(
    dim: raw::c_int,
    tag: raw::c_int,
    val: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoMeshSetReverse(dim, tag, val, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_mesh_set_algorithm(
    dim: raw::c_int,
    tag: raw::c_int,
    val: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoMeshSetAlgorithm(dim, tag, val, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_geo_mesh_set_size_from_boundary(
    dim: raw::c_int,
    tag: raw::c_int,
    val: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelGeoMeshSetSizeFromBoundary(dim, tag, val, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

/* gmsh/model/occ namespace */

pub unsafe fn gmsh_model_occ_add_point(
    x: f64,
    y: f64,
    z: f64,
    mesh_size: f64,
    tag: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddPoint(x, y, z, mesh_size, tag, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_line(
    start_tag: raw::c_int,
    end_tag: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddLine(start_tag, end_tag, tag, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_circle_arc(
    start_tag: raw::c_int,
    middle_tag: raw::c_int,
    end_tag: raw::c_int,
    tag: raw::c_int,
    center: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddCircleArc(start_tag, middle_tag, end_tag, tag, center, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_circle(
    x: f64,
    y: f64,
    z: f64,
    r: f64,
    tag: raw::c_int,
    angle1: f64,
    angle2: f64,
    z_axis: *const f64,
    z_axis_n: usize,
    x_axis: *const f64,
    x_axis_n: usize,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddCircle(
            x, y, z, r, tag, angle1, angle2, z_axis, z_axis_n, x_axis, x_axis_n, ierr,
        )
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_ellipse_arc(
    start_tag: raw::c_int,
    center_tag: raw::c_int,
    major_tag: raw::c_int,
    end_tag: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddEllipseArc(start_tag, center_tag, major_tag, end_tag, tag, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_ellipse(
    x: f64,
    y: f64,
    z: f64,
    r1: f64,
    r2: f64,
    tag: raw::c_int,
    angle1: f64,
    angle2: f64,
    z_axis: *const f64,
    z_axis_n: usize,
    x_axis: *const f64,
    x_axis_n: usize,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddEllipse(
            x, y, z, r1, r2, tag, angle1, angle2, z_axis, z_axis_n, x_axis, x_axis_n, ierr,
        )
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_spline(
    point_tags: *const raw::c_int,
    point_tags_n: usize,
    tag: raw::c_int,
    tangents: *const f64,
    tangents_n: usize,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddSpline(point_tags, point_tags_n, tag, tangents, tangents_n, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_bspline(
    point_tags: *const raw::c_int,
    point_tags_n: usize,
    tag: raw::c_int,
    degree: raw::c_int,
    weights: *const f64,
    weights_n: usize,
    knots: *const f64,
    knots_n: usize,
    multiplicities: *const raw::c_int,
    multiplicities_n: usize,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddBSpline(
            point_tags,
            point_tags_n,
            tag,
            degree,
            weights,
            weights_n,
            knots,
            knots_n,
            multiplicities,
            multiplicities_n,
            ierr,
        )
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_bezier(
    point_tags: *const raw::c_int,
    point_tags_n: usize,
    tag: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddBezier(point_tags, point_tags_n, tag, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_wire(
    curve_tags: *const raw::c_int,
    curve_tags_n: usize,
    tag: raw::c_int,
    check_closed: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddWire(curve_tags, curve_tags_n, tag, check_closed, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_curve_loop(
    curve_tags: *const raw::c_int,
    curve_tags_n: usize,
    tag: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddCurveLoop(curve_tags, curve_tags_n, tag, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_rectangle(
    x: f64,
    y: f64,
    z: f64,
    dx: f64,
    dy: f64,
    tag: raw::c_int,
    rounded_radius: f64,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddRectangle(x, y, z, dx, dy, tag, rounded_radius, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_disk(
    xc: f64,
    yc: f64,
    zc: f64,
    rx: f64,
    ry: f64,
    tag: raw::c_int,
    z_axis: *const f64,
    z_axis_n: usize,
    x_axis: *const f64,
    x_axis_n: usize,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddDisk(
            xc, yc, zc, rx, ry, tag, z_axis, z_axis_n, x_axis, x_axis_n, ierr,
        )
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_plane_surface(
    wire_tags: *const raw::c_int,
    wire_tags_n: usize,
    tag: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddPlaneSurface(wire_tags, wire_tags_n, tag, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_surface_filling(
    wire_tag: raw::c_int,
    tag: raw::c_int,
    point_tags: *const raw::c_int,
    point_tags_n: usize,
    degree: raw::c_int,
    num_points_on_curves: raw::c_int,
    num_iter: raw::c_int,
    anisotropic: raw::c_int,
    tol2d: f64,
    tol3d: f64,
    tol_ang: f64,
    tol_curv: f64,
    max_degree: raw::c_int,
    max_segments: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddSurfaceFilling(
            wire_tag,
            tag,
            point_tags,
            point_tags_n,
            degree,
            num_points_on_curves,
            num_iter,
            anisotropic,
            tol2d,
            tol3d,
            tol_ang,
            tol_curv,
            max_degree,
            max_segments,
            ierr,
        )
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_bspline_filling(
    wire_tag: raw::c_int,
    tag: raw::c_int,
    type_: *const raw::c_char,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddBSplineFilling(wire_tag, tag, type_, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_bezier_filling(
    wire_tag: raw::c_int,
    tag: raw::c_int,
    type_: *const raw::c_char,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddBezierFilling(wire_tag, tag, type_, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_bspline_surface(
    point_tags: *const raw::c_int,
    point_tags_n: usize,
    num_points_u: raw::c_int,
    tag: raw::c_int,
    degree_u: raw::c_int,
    degree_v: raw::c_int,
    weights: *const f64,
    weights_n: usize,
    knots_u: *const f64,
    knots_u_n: usize,
    knots_v: *const f64,
    knots_v_n: usize,
    multiplicities_u: *const raw::c_int,
    multiplicities_u_n: usize,
    multiplicities_v: *const raw::c_int,
    multiplicities_v_n: usize,
    wire_tags: *const raw::c_int,
    wire_tags_n: usize,
    wire_3d: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddBSplineSurface(
            point_tags,
            point_tags_n,
            num_points_u,
            tag,
            degree_u,
            degree_v,
            weights,
            weights_n,
            knots_u,
            knots_u_n,
            knots_v,
            knots_v_n,
            multiplicities_u,
            multiplicities_u_n,
            multiplicities_v,
            multiplicities_v_n,
            wire_tags,
            wire_tags_n,
            wire_3d,
            ierr,
        )
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_bezier_surface(
    point_tags: *const raw::c_int,
    point_tags_n: usize,
    num_points_u: raw::c_int,
    tag: raw::c_int,
    wire_tags: *const raw::c_int,
    wire_tags_n: usize,
    wire_3d: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddBezierSurface(
            point_tags,
            point_tags_n,
            num_points_u,
            tag,
            wire_tags,
            wire_tags_n,
            wire_3d,
            ierr,
        )
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_trimmed_surface(
    surface_tag: raw::c_int,
    wire_tags: *const raw::c_int,
    wire_tags_n: usize,
    wire_3d: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddTrimmedSurface(surface_tag, wire_tags, wire_tags_n, wire_3d, tag, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_surface_loop(
    surface_tags: *const raw::c_int,
    surface_tags_n: usize,
    tag: raw::c_int,
    sewing: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddSurfaceLoop(surface_tags, surface_tags_n, tag, sewing, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_volume(
    shell_tags: *const raw::c_int,
    shell_tags_n: usize,
    tag: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddVolume(shell_tags, shell_tags_n, tag, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_sphere(
    xc: f64,
    yc: f64,
    zc: f64,
    radius: f64,
    tag: raw::c_int,
    angle1: f64,
    angle2: f64,
    angle3: f64,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddSphere(xc, yc, zc, radius, tag, angle1, angle2, angle3, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_box(
    x: f64,
    y: f64,
    z: f64,
    dx: f64,
    dy: f64,
    dz: f64,
    tag: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddBox(x, y, z, dx, dy, dz, tag, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_cylinder(
    x: f64,
    y: f64,
    z: f64,
    dx: f64,
    dy: f64,
    dz: f64,
    r: f64,
    tag: raw::c_int,
    angle: f64,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddCylinder(x, y, z, dx, dy, dz, r, tag, angle, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_cone(
    x: f64,
    y: f64,
    z: f64,
    dx: f64,
    dy: f64,
    dz: f64,
    r1: f64,
    r2: f64,
    tag: raw::c_int,
    angle: f64,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddCone(x, y, z, dx, dy, dz, r1, r2, tag, angle, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_wedge(
    x: f64,
    y: f64,
    z: f64,
    dx: f64,
    dy: f64,
    dz: f64,
    tag: raw::c_int,
    ltx: f64,
    z_axis: *const f64,
    z_axis_n: usize,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddWedge(x, y, z, dx, dy, dz, tag, ltx, z_axis, z_axis_n, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_torus(
    x: f64,
    y: f64,
    z: f64,
    r1: f64,
    r2: f64,
    tag: raw::c_int,
    angle: f64,
    z_axis: *const f64,
    z_axis_n: usize,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddTorus(x, y, z, r1, r2, tag, angle, z_axis, z_axis_n, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_thru_sections(
    wire_tags: *const raw::c_int,
    wire_tags_n: usize,
    tag: raw::c_int,
    make_solid: raw::c_int,
    make_ruled: raw::c_int,
    max_degree: raw::c_int,
    continuity: *const raw::c_char,
    parametrization: *const raw::c_char,
    smoothing: raw::c_int,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut out_dim_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut out_dim_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let out_dim_tags = &mut out_dim_tags as *mut *mut raw::c_int;
        let out_dim_tags_n = &mut out_dim_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddThruSections(
            wire_tags,
            wire_tags_n,
            out_dim_tags,
            out_dim_tags_n,
            tag,
            make_solid,
            make_ruled,
            max_degree,
            continuity,
            parametrization,
            smoothing,
            ierr,
        );
    };

    if err == 0 {
        Ok((out_dim_tags, out_dim_tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_thick_solid(
    volume_tag: raw::c_int,
    exclude_surface_tags: *const raw::c_int,
    exclude_surface_tags_n: usize,
    offset: f64,
    tag: raw::c_int,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut out_dim_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut out_dim_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let out_dim_tags = &mut out_dim_tags as *mut *mut raw::c_int;
        let out_dim_tags_n = &mut out_dim_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddThickSolid(
            volume_tag,
            exclude_surface_tags,
            exclude_surface_tags_n,
            offset,
            out_dim_tags,
            out_dim_tags_n,
            tag,
            ierr,
        );
    };

    if err == 0 {
        Ok((out_dim_tags, out_dim_tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_extrude(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    dx: f64,
    dy: f64,
    dz: f64,
    num_elements: *const raw::c_int,
    num_elements_n: usize,
    heights: *const f64,
    heights_n: usize,
    recombine: raw::c_int,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut out_dim_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut out_dim_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let out_dim_tags = &mut out_dim_tags as *mut *mut raw::c_int;
        let out_dim_tags_n = &mut out_dim_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccExtrude(
            dim_tags,
            dim_tags_n,
            dx,
            dy,
            dz,
            out_dim_tags,
            out_dim_tags_n,
            num_elements,
            num_elements_n,
            heights,
            heights_n,
            recombine,
            ierr,
        );
    };

    if err == 0 {
        Ok((out_dim_tags, out_dim_tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_revolve(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    x: f64,
    y: f64,
    z: f64,
    ax: f64,
    ay: f64,
    az: f64,
    angle: f64,
    num_elements: *const raw::c_int,
    num_elements_n: usize,
    heights: *const f64,
    heights_n: usize,
    recombine: raw::c_int,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut out_dim_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut out_dim_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let out_dim_tags = &mut out_dim_tags as *mut *mut raw::c_int;
        let out_dim_tags_n = &mut out_dim_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccRevolve(
            dim_tags,
            dim_tags_n,
            x,
            y,
            z,
            ax,
            ay,
            az,
            angle,
            out_dim_tags,
            out_dim_tags_n,
            num_elements,
            num_elements_n,
            heights,
            heights_n,
            recombine,
            ierr,
        );
    };

    if err == 0 {
        Ok((out_dim_tags, out_dim_tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_add_pipe(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    wire_tag: raw::c_int,
    trihedron: *const raw::c_char,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut out_dim_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut out_dim_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let out_dim_tags = &mut out_dim_tags as *mut *mut raw::c_int;
        let out_dim_tags_n = &mut out_dim_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAddPipe(
            dim_tags,
            dim_tags_n,
            wire_tag,
            out_dim_tags,
            out_dim_tags_n,
            trihedron,
            ierr,
        );
    };

    if err == 0 {
        Ok((out_dim_tags, out_dim_tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_fillet(
    volume_tags: *const raw::c_int,
    volume_tags_n: usize,
    curve_tags: *const raw::c_int,
    curve_tags_n: usize,
    radii: *const f64,
    radii_n: usize,
    remove_volume: raw::c_int,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut out_dim_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut out_dim_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let out_dim_tags = &mut out_dim_tags as *mut *mut raw::c_int;
        let out_dim_tags_n = &mut out_dim_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccFillet(
            volume_tags,
            volume_tags_n,
            curve_tags,
            curve_tags_n,
            radii,
            radii_n,
            out_dim_tags,
            out_dim_tags_n,
            remove_volume,
            ierr,
        );
    };

    if err == 0 {
        Ok((out_dim_tags, out_dim_tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_chamfer(
    volume_tags: *const raw::c_int,
    volume_tags_n: usize,
    curve_tags: *const raw::c_int,
    curve_tags_n: usize,
    surface_tags: *const raw::c_int,
    surface_tags_n: usize,
    distances: *const f64,
    distances_n: usize,
    remove_volume: raw::c_int,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut out_dim_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut out_dim_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let out_dim_tags = &mut out_dim_tags as *mut *mut raw::c_int;
        let out_dim_tags_n = &mut out_dim_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccChamfer(
            volume_tags,
            volume_tags_n,
            curve_tags,
            curve_tags_n,
            surface_tags,
            surface_tags_n,
            distances,
            distances_n,
            out_dim_tags,
            out_dim_tags_n,
            remove_volume,
            ierr,
        );
    };

    if err == 0 {
        Ok((out_dim_tags, out_dim_tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_defeature(
    volume_tags: *const raw::c_int,
    volume_tags_n: usize,
    surface_tags: *const raw::c_int,
    surface_tags_n: usize,
    remove_volume: raw::c_int,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut out_dim_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut out_dim_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let out_dim_tags = &mut out_dim_tags as *mut *mut raw::c_int;
        let out_dim_tags_n = &mut out_dim_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccDefeature(
            volume_tags,
            volume_tags_n,
            surface_tags,
            surface_tags_n,
            out_dim_tags,
            out_dim_tags_n,
            remove_volume,
            ierr,
        );
    };

    if err == 0 {
        Ok((out_dim_tags, out_dim_tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_fillet_2d(
    edge_tag1: raw::c_int,
    edge_tag2: raw::c_int,
    radius: f64,
    tag: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccFillet2D(edge_tag1, edge_tag2, radius, tag, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_chamfer_2d(
    edge_tag1: raw::c_int,
    edge_tag2: raw::c_int,
    distance1: f64,
    distance2: f64,
    tag: raw::c_int,
) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccChamfer2D(edge_tag1, edge_tag2, distance1, distance2, tag, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_offset_curve(
    curve_loop_tag: raw::c_int,
    offset: f64,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut out_dim_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut out_dim_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let out_dim_tags = &mut out_dim_tags as *mut *mut raw::c_int;
        let out_dim_tags_n = &mut out_dim_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccOffsetCurve(curve_loop_tag, offset, out_dim_tags, out_dim_tags_n, ierr);
    };

    if err == 0 {
        Ok((out_dim_tags, out_dim_tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_get_distance(
    dim1: raw::c_int,
    tag1: raw::c_int,
    dim2: raw::c_int,
    tag2: raw::c_int,
) -> GmshRawResult<(f64, f64, f64, f64, f64, f64, f64)> {
    let mut distance: f64 = 0.0;
    let mut x1: f64 = 0.0;
    let mut y1: f64 = 0.0;
    let mut z1: f64 = 0.0;
    let mut x2: f64 = 0.0;
    let mut y2: f64 = 0.0;
    let mut z2: f64 = 0.0;
    let mut err: raw::c_int = 0;

    unsafe {
        let distance = &mut distance as *mut f64;
        let x1 = &mut x1 as *mut f64;
        let y1 = &mut y1 as *mut f64;
        let z1 = &mut z1 as *mut f64;
        let x2 = &mut x2 as *mut f64;
        let y2 = &mut y2 as *mut f64;
        let z2 = &mut z2 as *mut f64;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccGetDistance(
            dim1, tag1, dim2, tag2, distance, x1, y1, z1, x2, y2, z2, ierr,
        );
    };

    if err == 0 {
        Ok((distance, x1, y1, z1, x2, y2, z2))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_fuse(
    object_dim_tags: *const raw::c_int,
    object_dim_tags_n: usize,
    tool_dim_tags: *const raw::c_int,
    tool_dim_tags_n: usize,
    tag: raw::c_int,
    remove_object: raw::c_int,
    remove_tool: raw::c_int,
) -> GmshRawResult<(
    (*const raw::c_int, usize),
    (*const *const raw::c_int, *const usize, usize),
)> {
    let mut out_dim_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut out_dim_tags_n: usize = 0;
    let mut out_dim_tags_map: *mut *mut raw::c_int = std::ptr::null_mut();
    let mut out_dim_tags_map_n: *mut usize = std::ptr::null_mut();
    let mut out_dim_tags_map_nn: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let out_dim_tags = &mut out_dim_tags as *mut *mut raw::c_int;
        let out_dim_tags_n = &mut out_dim_tags_n as *mut usize;
        let out_dim_tags_map = &mut out_dim_tags_map as *mut *mut *mut raw::c_int;
        let out_dim_tags_map_n = &mut out_dim_tags_map_n as *mut *mut usize;
        let out_dim_tags_map_nn = &mut out_dim_tags_map_nn as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccFuse(
            object_dim_tags,
            object_dim_tags_n,
            tool_dim_tags,
            tool_dim_tags_n,
            out_dim_tags,
            out_dim_tags_n,
            out_dim_tags_map,
            out_dim_tags_map_n,
            out_dim_tags_map_nn,
            tag,
            remove_object,
            remove_tool,
            ierr,
        );
    };

    if err == 0 {
        Ok((
            (out_dim_tags, out_dim_tags_n),
            (
                out_dim_tags_map as *const *const raw::c_int,
                out_dim_tags_map_n as *const usize,
                out_dim_tags_map_nn,
            ),
        ))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_intersect(
    object_dim_tags: *const raw::c_int,
    object_dim_tags_n: usize,
    tool_dim_tags: *const raw::c_int,
    tool_dim_tags_n: usize,
    tag: raw::c_int,
    remove_object: raw::c_int,
    remove_tool: raw::c_int,
) -> GmshRawResult<(
    (*const raw::c_int, usize),
    (*const *const raw::c_int, *const usize, usize),
)> {
    let mut out_dim_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut out_dim_tags_n: usize = 0;
    let mut out_dim_tags_map: *mut *mut raw::c_int = std::ptr::null_mut();
    let mut out_dim_tags_map_n: *mut usize = std::ptr::null_mut();
    let mut out_dim_tags_map_nn: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let out_dim_tags = &mut out_dim_tags as *mut *mut raw::c_int;
        let out_dim_tags_n = &mut out_dim_tags_n as *mut usize;
        let out_dim_tags_map = &mut out_dim_tags_map as *mut *mut *mut raw::c_int;
        let out_dim_tags_map_n = &mut out_dim_tags_map_n as *mut *mut usize;
        let out_dim_tags_map_nn = &mut out_dim_tags_map_nn as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccIntersect(
            object_dim_tags,
            object_dim_tags_n,
            tool_dim_tags,
            tool_dim_tags_n,
            out_dim_tags,
            out_dim_tags_n,
            out_dim_tags_map,
            out_dim_tags_map_n,
            out_dim_tags_map_nn,
            tag,
            remove_object,
            remove_tool,
            ierr,
        );
    };

    if err == 0 {
        Ok((
            (out_dim_tags, out_dim_tags_n),
            (
                out_dim_tags_map as *const *const raw::c_int,
                out_dim_tags_map_n as *const usize,
                out_dim_tags_map_nn,
            ),
        ))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_cut(
    object_dim_tags: *const raw::c_int,
    object_dim_tags_n: usize,
    tool_dim_tags: *const raw::c_int,
    tool_dim_tags_n: usize,
    tag: raw::c_int,
    remove_object: raw::c_int,
    remove_tool: raw::c_int,
) -> GmshRawResult<(
    (*const raw::c_int, usize),
    (*const *const raw::c_int, *const usize, usize),
)> {
    let mut out_dim_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut out_dim_tags_n: usize = 0;
    let mut out_dim_tags_map: *mut *mut raw::c_int = std::ptr::null_mut();
    let mut out_dim_tags_map_n: *mut usize = std::ptr::null_mut();
    let mut out_dim_tags_map_nn: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let out_dim_tags = &mut out_dim_tags as *mut *mut raw::c_int;
        let out_dim_tags_n = &mut out_dim_tags_n as *mut usize;
        let out_dim_tags_map = &mut out_dim_tags_map as *mut *mut *mut raw::c_int;
        let out_dim_tags_map_n = &mut out_dim_tags_map_n as *mut *mut usize;
        let out_dim_tags_map_nn = &mut out_dim_tags_map_nn as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccCut(
            object_dim_tags,
            object_dim_tags_n,
            tool_dim_tags,
            tool_dim_tags_n,
            out_dim_tags,
            out_dim_tags_n,
            out_dim_tags_map,
            out_dim_tags_map_n,
            out_dim_tags_map_nn,
            tag,
            remove_object,
            remove_tool,
            ierr,
        );
    };

    if err == 0 {
        Ok((
            (out_dim_tags, out_dim_tags_n),
            (
                out_dim_tags_map as *const *const raw::c_int,
                out_dim_tags_map_n as *const usize,
                out_dim_tags_map_nn,
            ),
        ))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_fragment(
    object_dim_tags: *const raw::c_int,
    object_dim_tags_n: usize,
    tool_dim_tags: *const raw::c_int,
    tool_dim_tags_n: usize,
    tag: raw::c_int,
    remove_object: raw::c_int,
    remove_tool: raw::c_int,
) -> GmshRawResult<(
    (*const raw::c_int, usize),
    (*const *const raw::c_int, *const usize, usize),
)> {
    let mut out_dim_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut out_dim_tags_n: usize = 0;
    let mut out_dim_tags_map: *mut *mut raw::c_int = std::ptr::null_mut();
    let mut out_dim_tags_map_n: *mut usize = std::ptr::null_mut();
    let mut out_dim_tags_map_nn: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let out_dim_tags = &mut out_dim_tags as *mut *mut raw::c_int;
        let out_dim_tags_n = &mut out_dim_tags_n as *mut usize;
        let out_dim_tags_map = &mut out_dim_tags_map as *mut *mut *mut raw::c_int;
        let out_dim_tags_map_n = &mut out_dim_tags_map_n as *mut *mut usize;
        let out_dim_tags_map_nn = &mut out_dim_tags_map_nn as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccFragment(
            object_dim_tags,
            object_dim_tags_n,
            tool_dim_tags,
            tool_dim_tags_n,
            out_dim_tags,
            out_dim_tags_n,
            out_dim_tags_map,
            out_dim_tags_map_n,
            out_dim_tags_map_nn,
            tag,
            remove_object,
            remove_tool,
            ierr,
        );
    };

    if err == 0 {
        Ok((
            (out_dim_tags, out_dim_tags_n),
            (
                out_dim_tags_map as *const *const raw::c_int,
                out_dim_tags_map_n as *const usize,
                out_dim_tags_map_nn,
            ),
        ))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_translate(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    dx: f64,
    dy: f64,
    dz: f64,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccTranslate(dim_tags, dim_tags_n, dx, dy, dz, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_rotate(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    x: f64,
    y: f64,
    z: f64,
    ax: f64,
    ay: f64,
    az: f64,
    angle: f64,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccRotate(dim_tags, dim_tags_n, x, y, z, ax, ay, az, angle, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_dilate(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    x: f64,
    y: f64,
    z: f64,
    a: f64,
    b: f64,
    c: f64,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccDilate(dim_tags, dim_tags_n, x, y, z, a, b, c, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_mirror(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccMirror(dim_tags, dim_tags_n, a, b, c, d, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_symmetrize(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccSymmetrize(dim_tags, dim_tags_n, a, b, c, d, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_affine_transform(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    affine_transform: *const f64,
    affine_transform_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccAffineTransform(
            dim_tags,
            dim_tags_n,
            affine_transform,
            affine_transform_n,
            ierr,
        );
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_copy(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut out_dim_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut out_dim_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let out_dim_tags = &mut out_dim_tags as *mut *mut raw::c_int;
        let out_dim_tags_n = &mut out_dim_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccCopy(dim_tags, dim_tags_n, out_dim_tags, out_dim_tags_n, ierr);
    };

    if err == 0 {
        Ok((out_dim_tags, out_dim_tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_remove(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    recursive: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccRemove(dim_tags, dim_tags_n, recursive, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_remove_all_duplicates() -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccRemoveAllDuplicates(ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_heal_shapes(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    tolerance: f64,
    fix_degenerated: raw::c_int,
    fix_small_edges: raw::c_int,
    fix_small_faces: raw::c_int,
    sew_faces: raw::c_int,
    make_solids: raw::c_int,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut out_dim_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut out_dim_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let out_dim_tags = &mut out_dim_tags as *mut *mut raw::c_int;
        let out_dim_tags_n = &mut out_dim_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccHealShapes(
            out_dim_tags,
            out_dim_tags_n,
            dim_tags,
            dim_tags_n,
            tolerance,
            fix_degenerated,
            fix_small_edges,
            fix_small_faces,
            sew_faces,
            make_solids,
            ierr,
        );
    };

    if err == 0 {
        Ok((out_dim_tags, out_dim_tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_convert_to_nurbs(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccConvertToNURBS(dim_tags, dim_tags_n, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_import_shapes(
    file_name: *const raw::c_char,
    highest_dim_only: raw::c_int,
    format: *const raw::c_char,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut out_dim_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut out_dim_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let out_dim_tags = &mut out_dim_tags as *mut *mut raw::c_int;
        let out_dim_tags_n = &mut out_dim_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccImportShapes(
            file_name,
            out_dim_tags,
            out_dim_tags_n,
            highest_dim_only,
            format,
            ierr,
        );
    };

    if err == 0 {
        Ok((out_dim_tags, out_dim_tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_import_shapes_native_pointer(
    shape: *const raw::c_void,
    highest_dim_only: raw::c_int,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut out_dim_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut out_dim_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let out_dim_tags = &mut out_dim_tags as *mut *mut raw::c_int;
        let out_dim_tags_n = &mut out_dim_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccImportShapesNativePointer(
            shape,
            out_dim_tags,
            out_dim_tags_n,
            highest_dim_only,
            ierr,
        );
    };

    if err == 0 {
        Ok((out_dim_tags, out_dim_tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_get_entities(
    dim: raw::c_int,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut dim_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut dim_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let dim_tags = &mut dim_tags as *mut *mut raw::c_int;
        let dim_tags_n = &mut dim_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccGetEntities(dim_tags, dim_tags_n, dim, ierr);
    };

    if err == 0 {
        Ok((dim_tags, dim_tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_get_entities_in_bounding_box(
    xmin: f64,
    ymin: f64,
    zmin: f64,
    xmax: f64,
    ymax: f64,
    zmax: f64,
    dim: raw::c_int,
) -> GmshRawResult<(*const raw::c_int, usize)> {
    let mut dim_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut dim_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let dim_tags = &mut dim_tags as *mut *mut raw::c_int;
        let dim_tags_n = &mut dim_tags_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccGetEntitiesInBoundingBox(
            xmin, ymin, zmin, xmax, ymax, zmax, dim_tags, dim_tags_n, dim, ierr,
        );
    };

    if err == 0 {
        Ok((dim_tags, dim_tags_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_get_bounding_box(
    dim: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<(f64, f64, f64, f64, f64, f64)> {
    let mut xmin: f64 = 0.0;
    let mut ymin: f64 = 0.0;
    let mut zmin: f64 = 0.0;
    let mut xmax: f64 = 0.0;
    let mut ymax: f64 = 0.0;
    let mut zmax: f64 = 0.0;
    let mut err: raw::c_int = 0;

    unsafe {
        let xmin = &mut xmin as *mut f64;
        let ymin = &mut ymin as *mut f64;
        let zmin = &mut zmin as *mut f64;
        let xmax = &mut xmax as *mut f64;
        let ymax = &mut ymax as *mut f64;
        let zmax = &mut zmax as *mut f64;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccGetBoundingBox(dim, tag, xmin, ymin, zmin, xmax, ymax, zmax, ierr);
    };

    if err == 0 {
        Ok((xmin, ymin, zmin, xmax, ymax, zmax))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_get_curve_loops(
    surface_tag: raw::c_int,
) -> GmshRawResult<(
    (*const raw::c_int, usize),
    (*const *const raw::c_int, *const usize, usize),
)> {
    let mut curve_loop_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut curve_loop_tags_n: usize = 0;
    let mut curve_tags: *mut *mut raw::c_int = std::ptr::null_mut();
    let mut curve_tags_n: *mut usize = std::ptr::null_mut();
    let mut curve_tags_nn: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let curve_loop_tags = &mut curve_loop_tags as *mut *mut raw::c_int;
        let curve_loop_tags_n = &mut curve_loop_tags_n as *mut usize;
        let curve_tags = &mut curve_tags as *mut *mut *mut raw::c_int;
        let curve_tags_n = &mut curve_tags_n as *mut *mut usize;
        let curve_tags_nn = &mut curve_tags_nn as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccGetCurveLoops(
            surface_tag,
            curve_loop_tags,
            curve_loop_tags_n,
            curve_tags,
            curve_tags_n,
            curve_tags_nn,
            ierr,
        );
    };

    if err == 0 {
        Ok((
            (curve_loop_tags, curve_loop_tags_n),
            (
                curve_tags as *const *const raw::c_int,
                curve_tags_n,
                curve_tags_nn,
            ),
        ))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_get_surface_loops(
    volume_tag: raw::c_int,
) -> GmshRawResult<(
    (*const raw::c_int, usize),
    (*const *const raw::c_int, *const usize, usize),
)> {
    let mut surface_loop_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut surface_loop_tags_n: usize = 0;
    let mut surface_tags: *mut *mut raw::c_int = std::ptr::null_mut();
    let mut surface_tags_n: *mut usize = std::ptr::null_mut();
    let mut surface_tags_nn: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let surface_loop_tags = &mut surface_loop_tags as *mut *mut raw::c_int;
        let surface_loop_tags_n = &mut surface_loop_tags_n as *mut usize;
        let surface_tags = &mut surface_tags as *mut *mut *mut raw::c_int;
        let surface_tags_n = &mut surface_tags_n as *mut *mut usize;
        let surface_tags_nn = &mut surface_tags_nn as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccGetSurfaceLoops(
            volume_tag,
            surface_loop_tags,
            surface_loop_tags_n,
            surface_tags,
            surface_tags_n,
            surface_tags_nn,
            ierr,
        );
    };

    if err == 0 {
        Ok((
            (surface_loop_tags, surface_loop_tags_n),
            (
                surface_tags as *const *const raw::c_int,
                surface_tags_n,
                surface_tags_nn,
            ),
        ))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_get_mass(dim: raw::c_int, tag: raw::c_int) -> GmshRawResult<f64> {
    let mut mass: f64 = 0.0;
    let mut err: raw::c_int = 0;

    unsafe {
        let mass = &mut mass as *mut f64;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccGetMass(dim, tag, mass, ierr);
    };

    if err == 0 {
        Ok(mass)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_get_center_of_mass(
    dim: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<(f64, f64, f64)> {
    let mut x: f64 = 0.0;
    let mut y: f64 = 0.0;
    let mut z: f64 = 0.0;
    let mut err: raw::c_int = 0;

    unsafe {
        let x = &mut x as *mut f64;
        let y = &mut y as *mut f64;
        let z = &mut z as *mut f64;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccGetCenterOfMass(dim, tag, x, y, z, ierr);
    };

    if err == 0 {
        Ok((x, y, z))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_get_matrix_of_inertia(
    dim: raw::c_int,
    tag: raw::c_int,
) -> GmshRawResult<(*const f64, usize)> {
    let mut mat: *mut f64 = std::ptr::null_mut();
    let mut mat_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let mat = &mut mat as *mut *mut f64;
        let mat_n = &mut mat_n as *mut usize;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccGetMatrixOfInertia(dim, tag, mat, mat_n, ierr);
    };

    if err == 0 {
        Ok((mat, mat_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_get_max_tag(dim: raw::c_int) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccGetMaxTag(dim, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_set_max_tag(
    dim: raw::c_int,
    max_tag: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccSetMaxTag(dim, max_tag, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_model_occ_synchronize() -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccSynchronize(ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

/* gmsh/model/occ/mesh namespace */

pub unsafe fn gmsh_model_occ_mesh_set_size(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    size: f64,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshModelOccMeshSetSize(dim_tags, dim_tags_n, size, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}
