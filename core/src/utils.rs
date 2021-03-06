use std::ffi::{CStr, CString};
use std::os::raw::c_char;

pub fn from_c_str(to: *const c_char) -> String {
    let c_str = unsafe { CStr::from_ptr(to) };
    match c_str.to_str() {
        Err(_) => "".to_string(),
        Ok(s) => s.to_string(),
    }
}

pub fn to_c_str(from: String) -> *mut c_char {
    CString::new(from).unwrap().into_raw()
}

pub fn free(s: *mut c_char) {
    if s.is_null() {
        return;
    }

    unsafe { CString::from_raw(s) };
}

pub fn from_u64_vec_le(src: &Vec<u64>) -> Vec<u32> {
    src.iter()
        .map(|x| ((x | 0) as u32, (x >> 32) as u32))
        .fold(Vec::new(), |mut res, (l, s)| {
            res.push(l);
            res.push(s);
            res
        })
}
