// SAFETY: most functions in this module are marked as unsafe due to their args

use std::os::raw;

use crate::result::{GmshRawError, GmshRawResult};
use crate::sys;

pub unsafe fn gmsh_plugin_set_number(
    name: *const raw::c_char,
    option: *const raw::c_char,
    value: f64,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe { sys::gmshPluginSetNumber(name, option, value, &mut err) };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_plugin_set_string(
    name: *const raw::c_char,
    option: *const raw::c_char,
    value: *const raw::c_char,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe { sys::gmshPluginSetString(name, option, value, &mut err) };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_plugin_run(name: *const raw::c_char) -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe { sys::gmshPluginRun(name, &mut err) };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}
