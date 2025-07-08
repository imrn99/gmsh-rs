// SAFETY: most functions in this module are marked as unsafe due to their args

use std::os::raw;

use crate::result::{GmshRawError, GmshRawResult};
use crate::sys;

pub unsafe fn gmsh_option_set_number(name: *const raw::c_char, value: f64) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshOptionSetNumber(name, value, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_option_get_number(name: *const raw::c_char) -> GmshRawResult<f64> {
    let mut value: f64 = 0.0;
    let mut err: raw::c_int = 0;

    unsafe {
        let v = &mut value as *mut f64;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshOptionGetNumber(name, v, ierr);
    };

    if err == 0 {
        Ok(value)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_option_set_string(
    name: *const raw::c_char,
    value: *const raw::c_char,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshOptionSetString(name, value, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_option_get_string(name: *const raw::c_char) -> GmshRawResult<*mut raw::c_char> {
    // God willing, their retarded API isn't going to make us deref into oblivion
    let mut value: *mut raw::c_char = std::ptr::null_mut();
    let mut err: raw::c_int = 0;

    unsafe {
        let v = (&mut value) as *mut *mut raw::c_char;
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshOptionGetString(name, v, ierr);
    };

    if err == 0 {
        Ok(value)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_option_set_color(
    name: *const raw::c_char,
    r: raw::c_int,
    g: raw::c_int,
    b: raw::c_int,
    a: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshOptionSetColor(name, r, g, b, a, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_option_get_color(name: *const raw::c_char) -> GmshRawResult<[raw::c_int; 4]> {
    // Seriously, just return the fucking thing, it's not even heap-allocated here
    let r: *mut raw::c_int = std::ptr::dangling_mut();
    let g: *mut raw::c_int = std::ptr::dangling_mut();
    let b: *mut raw::c_int = std::ptr::dangling_mut();
    let a: *mut raw::c_int = std::ptr::dangling_mut();
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshOptionGetColor(name, r, g, b, a, ierr);
    };

    if err == 0 {
        Ok(unsafe { [*r, *g, *b, *a] })
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_option_restore_defaults() -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshOptionRestoreDefaults(ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}
