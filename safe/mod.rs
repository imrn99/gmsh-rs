// introduce as much strong typing as possible

pub mod logger;

mod types;
mod utils;

use std::os::raw;

pub use types::*;

use crate::result;

/// Initialize the Gmsh API. This must be called before any call to the other functions in the API.
/// If `argc' and `argv' (or just `argv' in Python or Julia) are provided, they will be handled in
/// the same way as the command line arguments in the Gmsh app. If `readConfigFiles' is set, read
/// system Gmsh configuration files (gmshrc and gmsh-options). If `run' is set, run in the same way
/// as the Gmsh app, either interactively or in batch mode depending on the command line arguments.
/// If `run' is not set, initializing the API sets the options "General.AbortOnError" to 2 and
/// "General.Terminal" to 1.
pub fn gmsh_initialize(read_config_files: bool, run: bool) -> GmshResult<()> {
    let args: Vec<_> = std::env::args().collect();
    let mut raw_args: Vec<_> = args
        .into_iter()
        .map(|mut s| s.as_mut_ptr() as *mut raw::c_char)
        .collect();

    unsafe {
        result::gmsh_initialize(
            raw_args.len() as i32,
            raw_args.as_mut_ptr(),
            read_config_files as i32,
            run as i32,
        )?;
    }

    Ok(())
}

/// Return `true` if the Gmsh API is initialized, `false` if not.
pub fn gmsh_is_initialized() -> GmshResult<bool> {
    let res = unsafe { result::gmsh_is_initialized()? };
    Ok(!res == 0)
}
