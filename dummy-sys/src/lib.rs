use std::ffi::{CString, c_char, c_int};

unsafe extern "C" {
    fn dummy_lookup(key: *const c_char) -> c_int;
}

pub fn lookup(key: &str) -> i32 {
    let key = CString::new(key).unwrap();
    unsafe { dummy_lookup(key.as_ptr()) }
}
