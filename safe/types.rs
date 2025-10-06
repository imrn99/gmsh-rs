use std::os::raw;

use crate::{
    result::{GmshRawError, logger},
    safe::utils::string_from_ptr,
};

pub enum GmshError {
    RetOk { code: i32, msg: String },
    RetFail { code: i32, ret_code: i32 },
}

impl From<GmshRawError> for GmshError {
    fn from(value: GmshRawError) -> Self {
        let code = value.0;
        match unsafe { logger::gmsh_logger_get_last_error() } {
            Ok(chain) => {
                let msg = unsafe { string_from_ptr(chain) };
                Self::RetOk { code, msg }
            }
            Err(GmshRawError(c)) => Self::RetFail { code, ret_code: c },
        }
    }
}

pub type GmshResult<T> = Result<T, GmshError>;

pub struct Tag(pub i32);

pub enum Dim {
    All = -1,
    Vertex = 0,
    Edge = 1,
    Face = 2,
    Volume = 3,
}

impl TryFrom<raw::c_int> for Dim {
    type Error = &'static str;
    fn try_from(value: raw::c_int) -> Result<Self, Self::Error> {
        match value {
            -1 => Ok(Self::All),
            0 => Ok(Self::Vertex),
            1 => Ok(Self::Edge),
            2 => Ok(Self::Face),
            3 => Ok(Self::Volume),
            _ => Err("dim != ( -1 | 0 | 1 | 2 | 3 )"),
        }
    }
}

pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl From<(f64, f64, f64)> for Vertex {
    fn from((x, y, z): (f64, f64, f64)) -> Self {
        Self { x, y, z }
    }
}

impl From<Vertex> for (f64, f64, f64) {
    fn from(value: Vertex) -> Self {
        (value.x, value.y, value.z)
    }
}

pub struct BoundingBox {
    pub min: Vertex,
    pub max: Vertex,
}

impl From<((f64, f64, f64), (f64, f64, f64))> for BoundingBox {
    fn from((vmin, vmax): ((f64, f64, f64), (f64, f64, f64))) -> Self {
        Self {
            min: vmin.into(),
            max: vmax.into(),
        }
    }
}

pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl TryFrom<[raw::c_int; 4]> for Color {
    type Error = std::num::TryFromIntError;
    fn try_from([r, g, b, a]: [raw::c_int; 4]) -> Result<Self, Self::Error> {
        Ok(Self {
            r: u8::try_from(r)?,
            g: u8::try_from(g)?,
            b: u8::try_from(b)?,
            a: u8::try_from(a)?,
        })
    }
}

impl From<Color> for [raw::c_int; 4] {
    fn from(value: Color) -> Self {
        [
            value.r as raw::c_int,
            value.g as raw::c_int,
            value.b as raw::c_int,
            value.a as raw::c_int,
        ]
    }
}
