pub mod option;

pub mod model;

pub mod view;

pub mod plugin;

pub mod graphics;

pub mod fltk;

pub mod parser;

pub mod onelab;

pub mod logger;

// SAFETY: most functions in this module are marked as unsafe due to their args

use std::os::raw;

use crate::sys;

/// GMSH error value wrapper.
pub struct GmshRawError(pub raw::c_int);

pub type GmshRawResult<T> = Result<T, GmshRawError>;

/// Initialize the Gmsh API. This must be called before any call to the other functions in the API.
/// If `argc' and `argv' (or just `argv' in Python or Julia) are provided, they will be handled in
/// the same way as the command line arguments in the Gmsh app. If `readConfigFiles' is set, read
/// system Gmsh configuration files (gmshrc and gmsh-options). If `run' is set, run in the same way
/// as the Gmsh app, either interactively or in batch mode depending on the command line arguments.
/// If `run' is not set, initializing the API sets the options "General.AbortOnError" to 2 and
/// "General.Terminal" to 1.
pub unsafe fn gmsh_initialize(
    argc: raw::c_int,
    argv: *mut *mut raw::c_char,
    read_config_files: raw::c_int,
    run: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshInitialize(argc, argv, read_config_files, run, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

/// Return 1 if the Gmsh API is initialized, and 0 if not.
pub unsafe fn gmsh_is_initialized() -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshIsInitialized(ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

/// Finalize the Gmsh API. This must be called when you are done using the Gmsh API.
pub unsafe fn gmsh_finalize() -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshFinalize(ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_open(file_name: *const raw::c_char) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshOpen(file_name, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_merge(file_name: *const raw::c_char) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshMerge(file_name, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_write(file_name: *const raw::c_char) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshWrite(file_name, ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_clear() -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshClear(ierr);
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}
