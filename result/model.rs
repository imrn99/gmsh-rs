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

pub unsafe fn gmsh_model_mesh_generate(
    dim: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_partition(
    num_part: raw::c_int,
    element_tags: *const usize,
    element_tags_n: usize,
    partitions: *const raw::c_int,
    partitions_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_unpartition(ierr: *mut raw::c_int) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_optimize(
    method: *const raw::c_char,
    force: raw::c_int,
    niter: raw::c_int,
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_recombine(ierr: *mut raw::c_int) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_refine(ierr: *mut raw::c_int) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_set_order(
    order: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_last_entity_error(
    dim_tags: *mut *mut raw::c_int,
    dim_tags_n: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_last_node_error(
    node_tags: *mut *mut usize,
    node_tags_n: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_clear(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_remove_elements(
    dim: raw::c_int,
    tag: raw::c_int,
    element_tags: *const usize,
    element_tags_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_reverse(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_reverse_elements(
    element_tags: *const usize,
    element_tags_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_affine_transform(
    affine_transform: *const f64,
    affine_transform_n: usize,
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_nodes(
    node_tags: *mut *mut usize,
    node_tags_n: *mut usize,
    coord: *mut *mut f64,
    coord_n: *mut usize,
    parametric_coord: *mut *mut f64,
    parametric_coord_n: *mut usize,
    dim: raw::c_int,
    tag: raw::c_int,
    include_boundary: raw::c_int,
    return_parametric_coord: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_nodes_by_element_type(
    element_type: raw::c_int,
    node_tags: *mut *mut usize,
    node_tags_n: *mut usize,
    coord: *mut *mut f64,
    coord_n: *mut usize,
    parametric_coord: *mut *mut f64,
    parametric_coord_n: *mut usize,
    tag: raw::c_int,
    return_parametric_coord: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_node(
    node_tag: usize,
    coord: *mut *mut f64,
    coord_n: *mut usize,
    parametric_coord: *mut *mut f64,
    parametric_coord_n: *mut usize,
    dim: *mut raw::c_int,
    tag: *mut raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_set_node(
    node_tag: usize,
    coord: *const f64,
    coord_n: usize,
    parametric_coord: *const f64,
    parametric_coord_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_rebuild_node_cache(
    only_if_necessary: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_rebuild_element_cache(
    only_if_necessary: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_nodes_for_physical_group(
    dim: raw::c_int,
    tag: raw::c_int,
    node_tags: *mut *mut usize,
    node_tags_n: *mut usize,
    coord: *mut *mut f64,
    coord_n: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_max_node_tag(
    max_tag: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
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
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_reclassify_nodes(ierr: *mut raw::c_int) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_relocate_nodes(
    dim: raw::c_int,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_elements(
    element_types: *mut *mut raw::c_int,
    element_types_n: *mut usize,
    element_tags: *mut *mut *mut usize,
    element_tags_n: *mut *mut usize,
    element_tags_nn: *mut usize,
    node_tags: *mut *mut *mut usize,
    node_tags_n: *mut *mut usize,
    node_tags_nn: *mut usize,
    dim: raw::c_int,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_element(
    element_tag: usize,
    element_type: *mut raw::c_int,
    node_tags: *mut *mut usize,
    node_tags_n: *mut usize,
    dim: *mut raw::c_int,
    tag: *mut raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_element_by_coordinates(
    x: f64,
    y: f64,
    z: f64,
    element_tag: *mut usize,
    element_type: *mut raw::c_int,
    node_tags: *mut *mut usize,
    node_tags_n: *mut usize,
    u: *mut f64,
    v: *mut f64,
    w: *mut f64,
    dim: raw::c_int,
    strict: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_elements_by_coordinates(
    x: f64,
    y: f64,
    z: f64,
    element_tags: *mut *mut usize,
    element_tags_n: *mut usize,
    dim: raw::c_int,
    strict: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_local_coordinates_in_element(
    element_tag: usize,
    x: f64,
    y: f64,
    z: f64,
    u: *mut f64,
    v: *mut f64,
    w: *mut f64,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_element_types(
    element_types: *mut *mut raw::c_int,
    element_types_n: *mut usize,
    dim: raw::c_int,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_element_type(
    family_name: *const raw::c_char,
    order: raw::c_int,
    serendip: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_mesh_get_element_properties(
    element_type: raw::c_int,
    element_name: *mut *mut raw::c_char,
    dim: *mut raw::c_int,
    order: *mut raw::c_int,
    num_nodes: *mut raw::c_int,
    local_node_coord: *mut *mut f64,
    local_node_coord_n: *mut usize,
    num_primary_nodes: *mut raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_elements_by_type(
    element_type: raw::c_int,
    element_tags: *mut *mut usize,
    element_tags_n: *mut usize,
    node_tags: *mut *mut usize,
    node_tags_n: *mut usize,
    tag: raw::c_int,
    task: usize,
    num_tasks: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_max_element_tag(
    max_tag: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_preallocate_elements_by_type(
    element_type: raw::c_int,
    element_tag: raw::c_int,
    node_tag: raw::c_int,
    element_tags: *mut *mut usize,
    element_tags_n: *mut usize,
    node_tags: *mut *mut usize,
    node_tags_n: *mut usize,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_element_qualities(
    element_tags: *const usize,
    element_tags_n: usize,
    elements_quality: *mut *mut f64,
    elements_quality_n: *mut usize,
    quality_name: *const raw::c_char,
    task: usize,
    num_tasks: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
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
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_add_elements_by_type(
    tag: raw::c_int,
    element_type: raw::c_int,
    element_tags: *const usize,
    element_tags_n: usize,
    node_tags: *const usize,
    node_tags_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_integration_points(
    element_type: raw::c_int,
    integration_type: *const raw::c_char,
    local_coord: *mut *mut f64,
    local_coord_n: *mut usize,
    weights: *mut *mut f64,
    weights_n: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_jacobians(
    element_type: raw::c_int,
    local_coord: *const f64,
    local_coord_n: usize,
    jacobians: *mut *mut f64,
    jacobians_n: *mut usize,
    determinants: *mut *mut f64,
    determinants_n: *mut usize,
    coord: *mut *mut f64,
    coord_n: *mut usize,
    tag: raw::c_int,
    task: usize,
    num_tasks: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_preallocate_jacobians(
    element_type: raw::c_int,
    num_evaluation_points: raw::c_int,
    allocate_jacobians: raw::c_int,
    allocate_determinants: raw::c_int,
    allocate_coord: raw::c_int,
    jacobians: *mut *mut f64,
    jacobians_n: *mut usize,
    determinants: *mut *mut f64,
    determinants_n: *mut usize,
    coord: *mut *mut f64,
    coord_n: *mut usize,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_jacobian(
    element_tag: usize,
    local_coord: *const f64,
    local_coord_n: usize,
    jacobians: *mut *mut f64,
    jacobians_n: *mut usize,
    determinants: *mut *mut f64,
    determinants_n: *mut usize,
    coord: *mut *mut f64,
    coord_n: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_basis_functions(
    element_type: raw::c_int,
    local_coord: *const f64,
    local_coord_n: usize,
    function_space_type: *const raw::c_char,
    num_components: *mut raw::c_int,
    basis_functions: *mut *mut f64,
    basis_functions_n: *mut usize,
    num_orientations: *mut raw::c_int,
    wanted_orientations: *const raw::c_int,
    wanted_orientations_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_basis_functions_orientation(
    element_type: raw::c_int,
    function_space_type: *const raw::c_char,
    basis_functions_orientation: *mut *mut raw::c_int,
    basis_functions_orientation_n: *mut usize,
    tag: raw::c_int,
    task: usize,
    num_tasks: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_basis_functions_orientation_for_element(
    element_tag: usize,
    function_space_type: *const raw::c_char,
    basis_functions_orientation: *mut raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_number_of_orientations(
    element_type: raw::c_int,
    function_space_type: *const raw::c_char,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_mesh_preallocate_basis_functions_orientation(
    element_type: raw::c_int,
    basis_functions_orientation: *mut *mut raw::c_int,
    basis_functions_orientation_n: *mut usize,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_edges(
    node_tags: *const usize,
    node_tags_n: usize,
    edge_tags: *mut *mut usize,
    edge_tags_n: *mut usize,
    edge_orientations: *mut *mut raw::c_int,
    edge_orientations_n: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_faces(
    face_type: raw::c_int,
    node_tags: *const usize,
    node_tags_n: usize,
    face_tags: *mut *mut usize,
    face_tags_n: *mut usize,
    face_orientations: *mut *mut raw::c_int,
    face_orientations_n: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_create_edges(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_create_faces(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_all_edges(
    edge_tags: *mut *mut usize,
    edge_tags_n: *mut usize,
    edge_nodes: *mut *mut usize,
    edge_nodes_n: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_all_faces(
    face_type: raw::c_int,
    face_tags: *mut *mut usize,
    face_tags_n: *mut usize,
    face_nodes: *mut *mut usize,
    face_nodes_n: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_add_edges(
    edge_tags: *const usize,
    edge_tags_n: usize,
    edge_nodes: *const usize,
    edge_nodes_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_add_faces(
    face_type: raw::c_int,
    face_tags: *const usize,
    face_tags_n: usize,
    face_nodes: *const usize,
    face_nodes_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_keys(
    element_type: raw::c_int,
    function_space_type: *const raw::c_char,
    type_keys: *mut *mut raw::c_int,
    type_keys_n: *mut usize,
    entity_keys: *mut *mut usize,
    entity_keys_n: *mut usize,
    coord: *mut *mut f64,
    coord_n: *mut usize,
    tag: raw::c_int,
    return_coord: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_keys_for_element(
    element_tag: usize,
    function_space_type: *const raw::c_char,
    type_keys: *mut *mut raw::c_int,
    type_keys_n: *mut usize,
    entity_keys: *mut *mut usize,
    entity_keys_n: *mut usize,
    coord: *mut *mut f64,
    coord_n: *mut usize,
    return_coord: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_number_of_keys(
    element_type: raw::c_int,
    function_space_type: *const raw::c_char,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_mesh_get_keys_information(
    type_keys: *const raw::c_int,
    type_keys_n: usize,
    entity_keys: *const usize,
    entity_keys_n: usize,
    element_type: raw::c_int,
    function_space_type: *const raw::c_char,
    info_keys: *mut *mut raw::c_int,
    info_keys_n: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_barycenters(
    element_type: raw::c_int,
    tag: raw::c_int,
    fast: raw::c_int,
    primary: raw::c_int,
    barycenters: *mut *mut f64,
    barycenters_n: *mut usize,
    task: usize,
    num_tasks: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_preallocate_barycenters(
    element_type: raw::c_int,
    barycenters: *mut *mut f64,
    barycenters_n: *mut usize,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_element_edge_nodes(
    element_type: raw::c_int,
    node_tags: *mut *mut usize,
    node_tags_n: *mut usize,
    tag: raw::c_int,
    primary: raw::c_int,
    task: usize,
    num_tasks: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_element_face_nodes(
    element_type: raw::c_int,
    face_type: raw::c_int,
    node_tags: *mut *mut usize,
    node_tags_n: *mut usize,
    tag: raw::c_int,
    primary: raw::c_int,
    task: usize,
    num_tasks: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_ghost_elements(
    dim: raw::c_int,
    tag: raw::c_int,
    element_tags: *mut *mut usize,
    element_tags_n: *mut usize,
    partitions: *mut *mut raw::c_int,
    partitions_n: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_set_size(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    size: f64,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_sizes(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    sizes: *mut *mut f64,
    sizes_n: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_set_size_at_parametric_points(
    dim: raw::c_int,
    tag: raw::c_int,
    parametric_coord: *const f64,
    parametric_coord_n: usize,
    sizes: *const f64,
    sizes_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
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
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_remove_size_callback(ierr: *mut raw::c_int) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_set_transfinite_curve(
    tag: raw::c_int,
    num_nodes: raw::c_int,
    mesh_type: *const raw::c_char,
    coef: f64,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_set_transfinite_surface(
    tag: raw::c_int,
    arrangement: *const raw::c_char,
    corner_tags: *const raw::c_int,
    corner_tags_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_set_transfinite_volume(
    tag: raw::c_int,
    corner_tags: *const raw::c_int,
    corner_tags_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_set_transfinite_automatic(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    corner_angle: f64,
    recombine: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_set_recombine(
    dim: raw::c_int,
    tag: raw::c_int,
    angle: f64,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_set_smoothing(
    dim: raw::c_int,
    tag: raw::c_int,
    val: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_set_reverse(
    dim: raw::c_int,
    tag: raw::c_int,
    val: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_set_algorithm(
    dim: raw::c_int,
    tag: raw::c_int,
    val: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_set_size_from_boundary(
    dim: raw::c_int,
    tag: raw::c_int,
    val: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_set_compound(
    dim: raw::c_int,
    tags: *const raw::c_int,
    tags_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_set_outward_orientation(
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_remove_constraints(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_embed(
    dim: raw::c_int,
    tags: *const raw::c_int,
    tags_n: usize,
    in_dim: raw::c_int,
    in_tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_remove_embedded(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    dim: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_embedded(
    dim: raw::c_int,
    tag: raw::c_int,
    dim_tags: *mut *mut raw::c_int,
    dim_tags_n: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_reorder_elements(
    element_type: raw::c_int,
    tag: raw::c_int,
    ordering: *const usize,
    ordering_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_compute_renumbering(
    old_tags: *mut *mut usize,
    old_tags_n: *mut usize,
    new_tags: *mut *mut usize,
    new_tags_n: *mut usize,
    method: *const raw::c_char,
    element_tags: *const usize,
    element_tags_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_renumber_nodes(
    old_tags: *const usize,
    old_tags_n: usize,
    new_tags: *const usize,
    new_tags_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_renumber_elements(
    old_tags: *const usize,
    old_tags_n: usize,
    new_tags: *const usize,
    new_tags_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_set_periodic(
    dim: raw::c_int,
    tags: *const raw::c_int,
    tags_n: usize,
    tags_master: *const raw::c_int,
    tags_master_n: usize,
    affine_transform: *const f64,
    affine_transform_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_periodic(
    dim: raw::c_int,
    tags: *const raw::c_int,
    tags_n: usize,
    tag_master: *mut *mut raw::c_int,
    tag_master_n: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_periodic_nodes(
    dim: raw::c_int,
    tag: raw::c_int,
    tag_master: *mut raw::c_int,
    node_tags: *mut *mut usize,
    node_tags_n: *mut usize,
    node_tags_master: *mut *mut usize,
    node_tags_master_n: *mut usize,
    affine_transform: *mut *mut f64,
    affine_transform_n: *mut usize,
    include_high_order_nodes: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_periodic_keys(
    element_type: raw::c_int,
    function_space_type: *const raw::c_char,
    tag: raw::c_int,
    tag_master: *mut raw::c_int,
    type_keys: *mut *mut raw::c_int,
    type_keys_n: *mut usize,
    type_keys_master: *mut *mut raw::c_int,
    type_keys_master_n: *mut usize,
    entity_keys: *mut *mut usize,
    entity_keys_n: *mut usize,
    entity_keys_master: *mut *mut usize,
    entity_keys_master_n: *mut usize,
    coord: *mut *mut f64,
    coord_n: *mut usize,
    coord_master: *mut *mut f64,
    coord_master_n: *mut usize,
    return_coord: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_import_stl(ierr: *mut raw::c_int) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_duplicate_nodes(
    tags: *mut *mut usize,
    tags_n: *mut usize,
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_remove_duplicate_nodes(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_remove_duplicate_elements(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_split_quadrangles(
    quality: f64,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_set_visibility(
    element_tags: *const usize,
    element_tags_n: usize,
    value: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_get_visibility(
    element_tags: *const usize,
    element_tags_n: usize,
    values: *mut *mut raw::c_int,
    values_n: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_classify_surfaces(
    angle: f64,
    boundary: raw::c_int,
    for_reparametrization: raw::c_int,
    curve_angle: f64,
    export_discrete: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_create_geometry(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_create_topology(
    make_simply_connected: raw::c_int,
    export_discrete: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_add_homology_request(
    type_: *const raw::c_char,
    domain_tags: *const raw::c_int,
    domain_tags_n: usize,
    subdomain_tags: *const raw::c_int,
    subdomain_tags_n: usize,
    dims: *const raw::c_int,
    dims_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_clear_homology_requests(ierr: *mut raw::c_int) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_compute_homology(
    dim_tags: *mut *mut raw::c_int,
    dim_tags_n: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_compute_cross_field(
    view_tags: *mut *mut raw::c_int,
    view_tags_n: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_triangulate(
    coord: *const f64,
    coord_n: usize,
    tri: *mut *mut usize,
    tri_n: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_tetrahedralize(
    coord: *const f64,
    coord_n: usize,
    tetra: *mut *mut usize,
    tetra_n: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

/* gmsh/model/mesh/field namespace */

pub unsafe fn gmsh_model_mesh_field_add(
    field_type: *const raw::c_char,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_mesh_field_remove(
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_field_list(
    tags: *mut *mut raw::c_int,
    tags_n: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_field_get_type(
    tag: raw::c_int,
    file_type: *mut *mut raw::c_char,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_field_set_number(
    tag: raw::c_int,
    option: *const raw::c_char,
    value: f64,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_field_get_number(
    tag: raw::c_int,
    option: *const raw::c_char,
    value: *mut f64,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_field_set_string(
    tag: raw::c_int,
    option: *const raw::c_char,
    value: *const raw::c_char,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_field_get_string(
    tag: raw::c_int,
    option: *const raw::c_char,
    value: *mut *mut raw::c_char,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_field_set_numbers(
    tag: raw::c_int,
    option: *const raw::c_char,
    values: *const f64,
    values_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_field_get_numbers(
    tag: raw::c_int,
    option: *const raw::c_char,
    values: *mut *mut f64,
    values_n: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_field_set_as_background_mesh(
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_mesh_field_set_as_boundary_layer(
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

/* gmsh/model/geo namespace */

pub unsafe fn gmsh_model_geo_add_point(
    x: f64,
    y: f64,
    z: f64,
    mesh_size: f64,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_geo_add_line(
    start_tag: raw::c_int,
    end_tag: raw::c_int,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_geo_add_circle_arc(
    start_tag: raw::c_int,
    center_tag: raw::c_int,
    end_tag: raw::c_int,
    tag: raw::c_int,
    nx: f64,
    ny: f64,
    nz: f64,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
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
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_geo_add_spline(
    point_tags: *const raw::c_int,
    point_tags_n: usize,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_geo_add_bspline(
    point_tags: *const raw::c_int,
    point_tags_n: usize,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_geo_add_bezier(
    point_tags: *const raw::c_int,
    point_tags_n: usize,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_geo_add_polyline(
    point_tags: *const raw::c_int,
    point_tags_n: usize,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_geo_add_compound_spline(
    curve_tags: *const raw::c_int,
    curve_tags_n: usize,
    num_intervals: raw::c_int,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_geo_add_compound_bspline(
    curve_tags: *const raw::c_int,
    curve_tags_n: usize,
    num_intervals: raw::c_int,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_geo_add_curve_loop(
    curve_tags: *const raw::c_int,
    curve_tags_n: usize,
    tag: raw::c_int,
    reorient: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_geo_add_curve_loops(
    curve_tags: *const raw::c_int,
    curve_tags_n: usize,
    tags: *mut *mut raw::c_int,
    tags_n: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_geo_add_plane_surface(
    wire_tags: *const raw::c_int,
    wire_tags_n: usize,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_geo_add_surface_filling(
    wire_tags: *const raw::c_int,
    wire_tags_n: usize,
    tag: raw::c_int,
    sphere_center_tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_geo_add_surface_loop(
    surface_tags: *const raw::c_int,
    surface_tags_n: usize,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_geo_add_volume(
    shell_tags: *const raw::c_int,
    shell_tags_n: usize,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_geo_add_geometry(
    geometry: *const raw::c_char,
    numbers: *const f64,
    numbers_n: usize,
    strings: *const *const raw::c_char,
    strings_n: usize,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_geo_add_point_on_geometry(
    geometry_tag: raw::c_int,
    x: f64,
    y: f64,
    z: f64,
    mesh_size: f64,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_geo_extrude(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    dx: f64,
    dy: f64,
    dz: f64,
    out_dim_tags: *mut *mut raw::c_int,
    out_dim_tags_n: *mut usize,
    num_elements: *const raw::c_int,
    num_elements_n: usize,
    heights: *const f64,
    heights_n: usize,
    recombine: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
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
    out_dim_tags: *mut *mut raw::c_int,
    out_dim_tags_n: *mut usize,
    num_elements: *const raw::c_int,
    num_elements_n: usize,
    heights: *const f64,
    heights_n: usize,
    recombine: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
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
    out_dim_tags: *mut *mut raw::c_int,
    out_dim_tags_n: *mut usize,
    num_elements: *const raw::c_int,
    num_elements_n: usize,
    heights: *const f64,
    heights_n: usize,
    recombine: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_geo_extrude_boundary_layer(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    out_dim_tags: *mut *mut raw::c_int,
    out_dim_tags_n: *mut usize,
    num_elements: *const raw::c_int,
    num_elements_n: usize,
    heights: *const f64,
    heights_n: usize,
    recombine: raw::c_int,
    second: raw::c_int,
    view_index: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_geo_translate(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    dx: f64,
    dy: f64,
    dz: f64,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
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
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
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
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_geo_mirror(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_geo_symmetrize(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_geo_copy(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    out_dim_tags: *mut *mut raw::c_int,
    out_dim_tags_n: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_geo_remove(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    recursive: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_geo_remove_all_duplicates(ierr: *mut raw::c_int) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_geo_split_curve(
    tag: raw::c_int,
    point_tags: *const raw::c_int,
    point_tags_n: usize,
    curve_tags: *mut *mut raw::c_int,
    curve_tags_n: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_geo_get_max_tag(
    dim: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_geo_set_max_tag(
    dim: raw::c_int,
    max_tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_geo_add_physical_group(
    dim: raw::c_int,
    tags: *const raw::c_int,
    tags_n: usize,
    tag: raw::c_int,
    name: *const raw::c_char,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_geo_remove_physical_groups(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_geo_synchronize(ierr: *mut raw::c_int) -> GmshRawResult<()> {
    Ok(())
}

/* gmsh/model/geo/mesh namespace */

pub unsafe fn gmsh_model_geo_mesh_set_size(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    size: f64,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_geo_mesh_set_transfinite_curve(
    tag: raw::c_int,
    n_points: raw::c_int,
    mesh_type: *const raw::c_char,
    coef: f64,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_geo_mesh_set_transfinite_surface(
    tag: raw::c_int,
    arrangement: *const raw::c_char,
    corner_tags: *const raw::c_int,
    corner_tags_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_geo_mesh_set_transfinite_volume(
    tag: raw::c_int,
    corner_tags: *const raw::c_int,
    corner_tags_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_geo_mesh_set_recombine(
    dim: raw::c_int,
    tag: raw::c_int,
    angle: f64,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_geo_mesh_set_smoothing(
    dim: raw::c_int,
    tag: raw::c_int,
    val: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_geo_mesh_set_reverse(
    dim: raw::c_int,
    tag: raw::c_int,
    val: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_geo_mesh_set_algorithm(
    dim: raw::c_int,
    tag: raw::c_int,
    val: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_geo_mesh_set_size_from_boundary(
    dim: raw::c_int,
    tag: raw::c_int,
    val: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

/* gmsh/model/occ namespace */

pub unsafe fn gmsh_model_occ_add_point(
    x: f64,
    y: f64,
    z: f64,
    mesh_size: f64,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_occ_add_line(
    start_tag: raw::c_int,
    end_tag: raw::c_int,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_occ_add_circle_arc(
    start_tag: raw::c_int,
    middle_tag: raw::c_int,
    end_tag: raw::c_int,
    tag: raw::c_int,
    center: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
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
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_occ_add_ellipse_arc(
    start_tag: raw::c_int,
    center_tag: raw::c_int,
    major_tag: raw::c_int,
    end_tag: raw::c_int,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
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
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_occ_add_spline(
    point_tags: *const raw::c_int,
    point_tags_n: usize,
    tag: raw::c_int,
    tangents: *const f64,
    tangents_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
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
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_occ_add_bezier(
    point_tags: *const raw::c_int,
    point_tags_n: usize,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_occ_add_wire(
    curve_tags: *const raw::c_int,
    curve_tags_n: usize,
    tag: raw::c_int,
    check_closed: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_occ_add_curve_loop(
    curve_tags: *const raw::c_int,
    curve_tags_n: usize,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_occ_add_rectangle(
    x: f64,
    y: f64,
    z: f64,
    dx: f64,
    dy: f64,
    tag: raw::c_int,
    rounded_radius: f64,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
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
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_occ_add_plane_surface(
    wire_tags: *const raw::c_int,
    wire_tags_n: usize,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
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
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_occ_add_bspline_filling(
    wire_tag: raw::c_int,
    tag: raw::c_int,
    type_: *const raw::c_char,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_occ_add_bezier_filling(
    wire_tag: raw::c_int,
    tag: raw::c_int,
    type_: *const raw::c_char,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
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
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_occ_add_bezier_surface(
    point_tags: *const raw::c_int,
    point_tags_n: usize,
    num_points_u: raw::c_int,
    tag: raw::c_int,
    wire_tags: *const raw::c_int,
    wire_tags_n: usize,
    wire_3d: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_occ_add_trimmed_surface(
    surface_tag: raw::c_int,
    wire_tags: *const raw::c_int,
    wire_tags_n: usize,
    wire_3d: raw::c_int,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_occ_add_surface_loop(
    surface_tags: *const raw::c_int,
    surface_tags_n: usize,
    tag: raw::c_int,
    sewing: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_occ_add_volume(
    shell_tags: *const raw::c_int,
    shell_tags_n: usize,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
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
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_occ_add_box(
    x: f64,
    y: f64,
    z: f64,
    dx: f64,
    dy: f64,
    dz: f64,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
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
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
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
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
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
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
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
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_occ_add_thru_sections(
    wire_tags: *const raw::c_int,
    wire_tags_n: usize,
    out_dim_tags: *mut *mut raw::c_int,
    out_dim_tags_n: *mut usize,
    tag: raw::c_int,
    make_solid: raw::c_int,
    make_ruled: raw::c_int,
    max_degree: raw::c_int,
    continuity: *const raw::c_char,
    parametrization: *const raw::c_char,
    smoothing: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_add_thick_solid(
    volume_tag: raw::c_int,
    exclude_surface_tags: *const raw::c_int,
    exclude_surface_tags_n: usize,
    offset: f64,
    out_dim_tags: *mut *mut raw::c_int,
    out_dim_tags_n: *mut usize,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_extrude(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    dx: f64,
    dy: f64,
    dz: f64,
    out_dim_tags: *mut *mut raw::c_int,
    out_dim_tags_n: *mut usize,
    num_elements: *const raw::c_int,
    num_elements_n: usize,
    heights: *const f64,
    heights_n: usize,
    recombine: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
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
    out_dim_tags: *mut *mut raw::c_int,
    out_dim_tags_n: *mut usize,
    num_elements: *const raw::c_int,
    num_elements_n: usize,
    heights: *const f64,
    heights_n: usize,
    recombine: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_add_pipe(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    wire_tag: raw::c_int,
    out_dim_tags: *mut *mut raw::c_int,
    out_dim_tags_n: *mut usize,
    trihedron: *const raw::c_char,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_fillet(
    volume_tags: *const raw::c_int,
    volume_tags_n: usize,
    curve_tags: *const raw::c_int,
    curve_tags_n: usize,
    radii: *const f64,
    radii_n: usize,
    out_dim_tags: *mut *mut raw::c_int,
    out_dim_tags_n: *mut usize,
    remove_volume: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
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
    out_dim_tags: *mut *mut raw::c_int,
    out_dim_tags_n: *mut usize,
    remove_volume: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_defeature(
    volume_tags: *const raw::c_int,
    volume_tags_n: usize,
    surface_tags: *const raw::c_int,
    surface_tags_n: usize,
    out_dim_tags: *mut *mut raw::c_int,
    out_dim_tags_n: *mut usize,
    remove_volume: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_fillet_2d(
    edge_tag1: raw::c_int,
    edge_tag2: raw::c_int,
    radius: f64,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_occ_chamfer_2d(
    edge_tag1: raw::c_int,
    edge_tag2: raw::c_int,
    distance1: f64,
    distance2: f64,
    tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_occ_offset_curve(
    curve_loop_tag: raw::c_int,
    offset: f64,
    out_dim_tags: *mut *mut raw::c_int,
    out_dim_tags_n: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_get_distance(
    dim1: raw::c_int,
    tag1: raw::c_int,
    dim2: raw::c_int,
    tag2: raw::c_int,
    distance: *mut f64,
    x1: *mut f64,
    y1: *mut f64,
    z1: *mut f64,
    x2: *mut f64,
    y2: *mut f64,
    z2: *mut f64,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_fuse(
    object_dim_tags: *const raw::c_int,
    object_dim_tags_n: usize,
    tool_dim_tags: *const raw::c_int,
    tool_dim_tags_n: usize,
    out_dim_tags: *mut *mut raw::c_int,
    out_dim_tags_n: *mut usize,
    out_dim_tags_map: *mut *mut *mut raw::c_int,
    out_dim_tags_map_n: *mut *mut usize,
    out_dim_tags_map_nn: *mut usize,
    tag: raw::c_int,
    remove_object: raw::c_int,
    remove_tool: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_intersect(
    object_dim_tags: *const raw::c_int,
    object_dim_tags_n: usize,
    tool_dim_tags: *const raw::c_int,
    tool_dim_tags_n: usize,
    out_dim_tags: *mut *mut raw::c_int,
    out_dim_tags_n: *mut usize,
    out_dim_tags_map: *mut *mut *mut raw::c_int,
    out_dim_tags_map_n: *mut *mut usize,
    out_dim_tags_map_nn: *mut usize,
    tag: raw::c_int,
    remove_object: raw::c_int,
    remove_tool: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_cut(
    object_dim_tags: *const raw::c_int,
    object_dim_tags_n: usize,
    tool_dim_tags: *const raw::c_int,
    tool_dim_tags_n: usize,
    out_dim_tags: *mut *mut raw::c_int,
    out_dim_tags_n: *mut usize,
    out_dim_tags_map: *mut *mut *mut raw::c_int,
    out_dim_tags_map_n: *mut *mut usize,
    out_dim_tags_map_nn: *mut usize,
    tag: raw::c_int,
    remove_object: raw::c_int,
    remove_tool: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_fragment(
    object_dim_tags: *const raw::c_int,
    object_dim_tags_n: usize,
    tool_dim_tags: *const raw::c_int,
    tool_dim_tags_n: usize,
    out_dim_tags: *mut *mut raw::c_int,
    out_dim_tags_n: *mut usize,
    out_dim_tags_map: *mut *mut *mut raw::c_int,
    out_dim_tags_map_n: *mut *mut usize,
    out_dim_tags_map_nn: *mut usize,
    tag: raw::c_int,
    remove_object: raw::c_int,
    remove_tool: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_translate(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    dx: f64,
    dy: f64,
    dz: f64,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
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
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
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
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_mirror(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_symmetrize(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    a: f64,
    b: f64,
    c: f64,
    d: f64,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_affine_transform(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    affine_transform: *const f64,
    affine_transform_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_copy(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    out_dim_tags: *mut *mut raw::c_int,
    out_dim_tags_n: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_remove(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    recursive: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_remove_all_duplicates(ierr: *mut raw::c_int) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_heal_shapes(
    out_dim_tags: *mut *mut raw::c_int,
    out_dim_tags_n: *mut usize,
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    tolerance: f64,
    fix_degenerated: raw::c_int,
    fix_small_edges: raw::c_int,
    fix_small_faces: raw::c_int,
    sew_faces: raw::c_int,
    make_solids: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_convert_to_nurbs(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_import_shapes(
    file_name: *const raw::c_char,
    out_dim_tags: *mut *mut raw::c_int,
    out_dim_tags_n: *mut usize,
    highest_dim_only: raw::c_int,
    format: *const raw::c_char,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_import_shapes_native_pointer(
    shape: *const raw::c_void,
    out_dim_tags: *mut *mut raw::c_int,
    out_dim_tags_n: *mut usize,
    highest_dim_only: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_get_entities(
    dim_tags: *mut *mut raw::c_int,
    dim_tags_n: *mut usize,
    dim: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_get_entities_in_bounding_box(
    xmin: f64,
    ymin: f64,
    zmin: f64,
    xmax: f64,
    ymax: f64,
    zmax: f64,
    dim_tags: *mut *mut raw::c_int,
    dim_tags_n: *mut usize,
    dim: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_get_bounding_box(
    dim: raw::c_int,
    tag: raw::c_int,
    xmin: *mut f64,
    ymin: *mut f64,
    zmin: *mut f64,
    xmax: *mut f64,
    ymax: *mut f64,
    zmax: *mut f64,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_get_curve_loops(
    surface_tag: raw::c_int,
    curve_loop_tags: *mut *mut raw::c_int,
    curve_loop_tags_n: *mut usize,
    curve_tags: *mut *mut *mut raw::c_int,
    curve_tags_n: *mut *mut usize,
    curve_tags_nn: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_get_surface_loops(
    volume_tag: raw::c_int,
    surface_loop_tags: *mut *mut raw::c_int,
    surface_loop_tags_n: *mut usize,
    surface_tags: *mut *mut *mut raw::c_int,
    surface_tags_n: *mut *mut usize,
    surface_tags_nn: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_get_mass(
    dim: raw::c_int,
    tag: raw::c_int,
    mass: *mut f64,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_get_center_of_mass(
    dim: raw::c_int,
    tag: raw::c_int,
    x: *mut f64,
    y: *mut f64,
    z: *mut f64,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_get_matrix_of_inertia(
    dim: raw::c_int,
    tag: raw::c_int,
    mat: *mut *mut f64,
    mat_n: *mut usize,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_get_max_tag(
    dim: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<raw::c_int> {
    Ok(0)
}

pub unsafe fn gmsh_model_occ_set_max_tag(
    dim: raw::c_int,
    max_tag: raw::c_int,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}

pub unsafe fn gmsh_model_occ_synchronize(ierr: *mut raw::c_int) -> GmshRawResult<()> {
    Ok(())
}

/* gmsh/model/occ/mesh namespace */

pub unsafe fn gmsh_model_occ_mesh_set_size(
    dim_tags: *const raw::c_int,
    dim_tags_n: usize,
    size: f64,
    ierr: *mut raw::c_int,
) -> GmshRawResult<()> {
    Ok(())
}
