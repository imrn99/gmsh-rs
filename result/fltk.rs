// SAFETY: most functions in this module are marked as unsafe due to their args

use std::os::raw;

use crate::result::{GmshRawError, GmshRawResult};
use crate::sys;

pub unsafe fn gmsh_fltk_initialize() -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshFltkInitialize(ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_fltk_finalize() -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshFltkFinalize(ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_fltk_wait(time: f64) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshFltkWait(time, ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_fltk_update() -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshFltkUpdate(ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_fltk_awake(action: *const raw::c_char) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshFltkAwake(action, ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_fltk_lock() -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshFltkLock(ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_fltk_unlock() -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshFltkUnlock(ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_fltk_run() -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshFltkRun(ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_fltk_is_available() -> GmshRawResult<raw::c_int> {
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshFltkIsAvailable(ierr)
    };

    if err == 0 {
        Ok(res)
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_fltk_select_entities(
    dim: raw::c_int,
) -> GmshRawResult<(raw::c_int, (*mut raw::c_int, usize))> {
    let mut dim_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut dim_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshFltkSelectEntities(&mut dim_tags, &mut dim_tags_n, dim, ierr)
    };

    if err == 0 {
        Ok((res, (dim_tags, dim_tags_n)))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_fltk_select_elements() -> GmshRawResult<(raw::c_int, (*mut usize, usize))> {
    let mut element_tags: *mut usize = std::ptr::null_mut();
    let mut element_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshFltkSelectElements(&mut element_tags, &mut element_tags_n, ierr)
    };

    if err == 0 {
        Ok((res, (element_tags, element_tags_n)))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_fltk_select_views() -> GmshRawResult<(raw::c_int, (*mut raw::c_int, usize))> {
    let mut view_tags: *mut raw::c_int = std::ptr::null_mut();
    let mut view_tags_n: usize = 0;
    let mut err: raw::c_int = 0;

    let res = unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshFltkSelectViews(&mut view_tags, &mut view_tags_n, ierr)
    };

    if err == 0 {
        Ok((res, (view_tags, view_tags_n)))
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_fltk_split_current_window(
    how: *const raw::c_char,
    ratio: f64,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshFltkSplitCurrentWindow(how, ratio, ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_fltk_set_current_window(window_index: raw::c_int) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshFltkSetCurrentWindow(window_index, ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_fltk_set_status_message(
    message: *const raw::c_char,
    graphics: raw::c_int,
) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshFltkSetStatusMessage(message, graphics, ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_fltk_show_context_window(dim: raw::c_int, tag: raw::c_int) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshFltkShowContextWindow(dim, tag, ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_fltk_open_tree_item(name: *const raw::c_char) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshFltkOpenTreeItem(name, ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}

pub unsafe fn gmsh_fltk_close_tree_item(name: *const raw::c_char) -> GmshRawResult<()> {
    let mut err: raw::c_int = 0;

    unsafe {
        let ierr = &mut err as *mut raw::c_int;
        sys::gmshFltkCloseTreeItem(name, ierr)
    };

    if err == 0 {
        Ok(())
    } else {
        Err(GmshRawError(err))
    }
}
