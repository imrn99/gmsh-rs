// SAFETY: most functions in this module are marked as unsafe due to their args

use std::os::raw;

use crate::result::{GmshRawError, GmshRawResult};
use crate::sys;

pub unsafe fn gmsh_logger_write(
    message: *const raw::c_char,
    level: *const raw::c_char,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshLoggerWrite(message, level, ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_logger_start() -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshLoggerStart(ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_logger_get() -> GmshRawResult<(*mut *mut raw::c_char, usize)> {
    let mut log: *mut *mut raw::c_char = std::ptr::null_mut();
    let mut log_n: usize = 0;
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshLoggerGet(&mut log, &mut log_n, ierr)
    };

    if err == 0 {
        Ok((log, log_n))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_logger_stop() -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshLoggerStop(ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_logger_get_wall_time() -> GmshRawResult<f64> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshLoggerGetWallTime(ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_logger_get_cpu_time() -> GmshRawResult<f64> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshLoggerGetCpuTime(ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_logger_get_memory() -> GmshRawResult<f64> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshLoggerGetMemory(ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_logger_get_total_memory() -> GmshRawResult<f64> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshLoggerGetTotalMemory(ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_logger_get_last_error() -> GmshRawResult<*mut raw::c_char> {
    let mut error: *mut raw::c_char = std::ptr::null_mut();
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshLoggerGetLastError(&mut error, ierr)
    };

    if err == 0 {
        Ok(error)
    } else {
        Err(GmshRawError(err))
    }
}
