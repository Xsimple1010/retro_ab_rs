use std::sync::Mutex;

use super::ffi_tools::get_str_from_ptr;

pub fn get_string_mutex_from_ptr(ptr: *const i8) -> Mutex<String> {
    Mutex::new(get_str_from_ptr(ptr))
}
