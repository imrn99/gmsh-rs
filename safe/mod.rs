// introduce as much strong typing as possible

use std::ffi::CStr;
use std::os::raw;

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

// we have to handle vec with actual copies because Gmsh has a retarded API that
// requires you to use their function (gmshFree) to deallocate returned data

unsafe fn vec_from_metadata<T: Copy>((ptr, len): (*const T, usize)) -> Vec<T> {
    unsafe { std::slice::from_raw_parts(ptr, len).to_vec() }
}

unsafe fn vec_vec_from_metadata<T: Copy>(
    (ptr_data, ptr_lens, len): (*const *const T, *const usize, usize),
) -> Vec<Vec<T>> {
    let mut res = Vec::with_capacity(len);
    unsafe {
        for i in 0..len {
            let p = ptr_data.add(i);
            let l = ptr_lens.add(i);
            res.push(std::slice::from_raw_parts(*p, *l).to_vec());
        }
    }
    res
}

// what a shitshow

unsafe fn string_from_ptr(ptr: *const raw::c_char) -> String {
    unsafe { CStr::from_ptr(ptr).to_string_lossy().to_string() }
}

unsafe fn string_vec_from_metadata((ptr, len): (*const *const raw::c_char, usize)) -> Vec<String> {
    let mut res = Vec::with_capacity(len);
    unsafe {
        for i in 0..len {
            let p = ptr.add(i);
            res.push(string_from_ptr(*p));
        }
    }
    res
}

unsafe fn string_vec_vec_from_metadata(
    (ptr_data, ptr_lens, len): (*const *const *const raw::c_char, *const usize, usize),
) -> Vec<Vec<String>> {
    let mut res = Vec::with_capacity(len);
    unsafe {
        // what a shitshow
        for i in 0..len {
            let p = ptr_data.add(i);
            let l = ptr_lens.add(i);
            res.push(string_vec_from_metadata((*p, *l)));
        }
    }
    res
}
