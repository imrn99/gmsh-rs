// SAFETY: most functions in this module are marked as unsafe due to their args

use std::os::raw;

use crate::result::{GmshRawError, GmshRawResult};
use crate::sys;

pub unsafe fn gmsh_graphics_draw() -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe { sys::gmshGraphicsDraw(&mut err) };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}
