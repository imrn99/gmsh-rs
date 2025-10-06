use std::ffi::CStr;
use std::os::raw;

// we have to handle vec with actual copies because Gmsh has a retarded API that
// requires you to use their function (gmshFree) to deallocate returned data

pub(crate) unsafe fn vec_from_metadata<T: Copy>((ptr, len): (*const T, usize)) -> Vec<T> {
    unsafe { std::slice::from_raw_parts(ptr, len).to_vec() }
}

pub(crate) unsafe fn vec_vec_from_metadata<T: Copy>(
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

pub(crate) unsafe fn string_from_ptr(ptr: *const raw::c_char) -> String {
    unsafe { CStr::from_ptr(ptr).to_string_lossy().to_string() }
}

pub(crate) unsafe fn string_vec_from_metadata(
    (ptr, len): (*const *const raw::c_char, usize),
) -> Vec<String> {
    let mut res = Vec::with_capacity(len);
    unsafe {
        for i in 0..len {
            let p = ptr.add(i);
            res.push(string_from_ptr(*p));
        }
    }
    res
}

pub(crate) unsafe fn string_vec_vec_from_metadata(
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
