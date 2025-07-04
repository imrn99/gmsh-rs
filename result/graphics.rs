// SAFETY: most functions in this module are marked as unsafe due to their args

use std::os::raw;

use crate::result::{GmshRawError, GmshRawResult};
use crate::sys;

pub unsafe fn gmsh_graphics_draw(ierr: *mut raw::c_int) {
    todo!()
}
