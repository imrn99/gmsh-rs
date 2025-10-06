use std::os::raw;

use crate::result::logger;
use crate::safe::utils::string_from_ptr;
use crate::safe::{GmshResult, utils::string_vec_from_metadata};

pub enum LogLevel {
    Info,
    Warning,
    Error,
}

impl From<LogLevel> for *const raw::c_char {
    fn from(value: LogLevel) -> Self {
        match value {
            LogLevel::Info => "info".as_ptr() as *const raw::c_char,
            LogLevel::Warning => "warning".as_ptr() as *const raw::c_char,
            LogLevel::Error => "error".as_ptr() as *const raw::c_char,
        }
    }
}

pub fn gmsh_logger_write(message: &str, level: LogLevel) -> GmshResult<()> {
    unsafe {
        logger::gmsh_logger_write(message.as_ptr() as *const raw::c_char, level.into())?;
    }
    Ok(())
}

pub fn gmsh_logger_start() -> GmshResult<()> {
    unsafe {
        logger::gmsh_logger_start()?;
    }
    Ok(())
}

pub fn gmsh_logger_get() -> GmshResult<Vec<String>> {
    unsafe {
        let (ptr, len) = logger::gmsh_logger_get()?;
        Ok(string_vec_from_metadata((
            ptr as *const *const raw::c_char,
            len,
        )))
    }
}

pub fn gmsh_logger_stop() -> GmshResult<()> {
    unsafe {
        logger::gmsh_logger_stop()?;
    }
    Ok(())
}

pub fn gmsh_logger_get_wall_time() -> GmshResult<f64> {
    Ok(unsafe { logger::gmsh_logger_get_wall_time()? })
}

pub fn gmsh_logger_get_cpu_time() -> GmshResult<f64> {
    Ok(unsafe { logger::gmsh_logger_get_cpu_time()? })
}

pub fn gmsh_logger_get_memory() -> GmshResult<f64> {
    Ok(unsafe { logger::gmsh_logger_get_memory()? })
}

pub fn gmsh_logger_get_total_memory() -> GmshResult<f64> {
    Ok(unsafe { logger::gmsh_logger_get_total_memory()? })
}

pub fn gmsh_logger_get_last_error() -> GmshResult<String> {
    unsafe {
        let ptr = logger::gmsh_logger_get_last_error()?;
        Ok(string_from_ptr(ptr as *const raw::c_char))
    }
}
