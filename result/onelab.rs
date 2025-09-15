// SAFETY: most functions in this module are marked as unsafe due to their args

use std::os::raw;

use crate::result::{GmshRawError, GmshRawResult};
use crate::sys;

pub unsafe fn gmsh_onelab_set(
    data: *const raw::c_char,
    format: *const raw::c_char,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshOnelabSet(data, format, ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_onelab_get(
    name: *const raw::c_char,
    format: *const raw::c_char,
) -> GmshRawResult<*mut raw::c_char> {
    let mut data: *mut raw::c_char = std::ptr::null_mut();
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshOnelabGet(&mut data, name, format, ierr)
    };

    if err == 0 {
        Ok(data)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_onelab_get_names(
    search: *const raw::c_char,
) -> GmshRawResult<(*mut *mut raw::c_char, usize)> {
    let mut names: *mut *mut raw::c_char = std::ptr::null_mut();
    let mut names_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshOnelabGetNames(&mut names, &mut names_n, search, ierr)
    };

    if err == 0 {
        Ok((names, names_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_onelab_set_number(
    name: *const raw::c_char,
    value: *const f64,
    value_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshOnelabSetNumber(name, value, value_n, ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_onelab_set_string(
    name: *const raw::c_char,
    value: *const *const raw::c_char,
    value_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshOnelabSetString(name, value, value_n, ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_onelab_get_number(name: *const raw::c_char) -> GmshRawResult<(*mut f64, usize)> {
    let mut value: *mut f64 = std::ptr::null_mut();
    let mut value_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshOnelabGetNumber(name, &mut value, &mut value_n, ierr)
    };

    if err == 0 {
        Ok((value, value_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_onelab_get_string(
    name: *const raw::c_char,
) -> GmshRawResult<(*mut *mut raw::c_char, usize)> {
    let mut value: *mut *mut raw::c_char = std::ptr::null_mut();
    let mut value_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshOnelabGetString(name, &mut value, &mut value_n, ierr)
    };

    if err == 0 {
        Ok((value, value_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_onelab_get_changed(name: *const raw::c_char) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshOnelabGetChanged(name, ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_onelab_set_changed(
    name: *const raw::c_char,
    value: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshOnelabSetChanged(name, value, ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_onelab_clear(name: *const raw::c_char) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshOnelabClear(name, ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_onelab_run(
    name: *const raw::c_char,
    command: *const raw::c_char,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshOnelabRun(name, command, ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}
