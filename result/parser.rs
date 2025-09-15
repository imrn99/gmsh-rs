// SAFETY: most functions in this module are marked as unsafe due to their args

use std::os::raw;

use crate::result::{GmshRawError, GmshRawResult};
use crate::sys;

pub unsafe fn gmsh_parser_get_names(
    search: *const raw::c_char,
) -> GmshRawResult<(*mut *mut raw::c_char, usize)> {
    let mut names: *mut *mut raw::c_char = std::ptr::null_mut();
    let mut names_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshParserGetNames(&mut names, &mut names_n, search, ierr)
    };

    if err == 0 {
        Ok((names, names_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_parser_set_number(
    name: *const raw::c_char,
    value: *const f64,
    value_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshParserSetNumber(name, value, value_n, ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_parser_set_string(
    name: *const raw::c_char,
    value: *const *const raw::c_char,
    value_n: usize,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshParserSetString(name, value, value_n, ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_parser_get_number(name: *const raw::c_char) -> GmshRawResult<(*mut f64, usize)> {
    let mut value: *mut f64 = std::ptr::null_mut();
    let mut value_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshParserGetNumber(name, &mut value, &mut value_n, ierr)
    };

    if err == 0 {
        Ok((value, value_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_parser_get_string(
    name: *const raw::c_char,
) -> GmshRawResult<(*mut *mut raw::c_char, usize)> {
    let mut value: *mut *mut raw::c_char = std::ptr::null_mut();
    let mut value_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshParserGetString(name, &mut value, &mut value_n, ierr)
    };

    if err == 0 {
        Ok((value, value_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_parser_clear(name: *const raw::c_char) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshParserClear(name, ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_parser_parse(file_name: *const raw::c_char) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshParserParse(file_name, ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}
