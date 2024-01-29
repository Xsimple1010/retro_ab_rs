use crate::binding_libretro::{
    retro_core_option_v2_definition, retro_core_options_v2, retro_core_options_v2_intl,
};
use std::{
    ffi::{c_char, CStr},
    path::PathBuf,
};

pub struct Values {
    pub value: String,
    pub label: String,
}

pub struct Options {
    pub key: String,
    pub desc: String,
    pub desc_categorized: String,
    pub info: String,
    pub info_categorized: String,
    pub category_key: String,
    pub values: Vec<Values>,
    pub default_value: String,
}

pub struct OptionManager {
    pub file_path: PathBuf,
    pub opts: Vec<Options>,
}

fn get_str_from_ptr(ptr: *const ::std::os::raw::c_char) -> String {
    if ptr.is_null() {
        return "".to_string();
    }

    let c_char_ptr: *mut c_char = ptr as *mut c_char;
    let c_str = unsafe { CStr::from_ptr(c_char_ptr) };
    let str_slice = c_str.to_str().unwrap();

    str::to_owned(str_slice)
}

pub fn convert_option_v2_intl(option_intl_v2: retro_core_options_v2_intl) -> OptionManager {
    let mut options_manager = OptionManager {
        file_path: PathBuf::from(""),
        opts: Vec::new(),
    };

    unsafe {
        let mut _current_options: Option<retro_core_options_v2> = None;

        if option_intl_v2.local.is_null() {
            let op: retro_core_options_v2 = *(option_intl_v2.us);
            let en = *(op.definitions as *mut [retro_core_option_v2_definition; 90]);

            for e in en {
                if !e.key.is_null() {
                    let key = get_str_from_ptr(e.key);
                    let default_value = get_str_from_ptr(e.default_value);
                    let info = get_str_from_ptr(e.info);
                    let desc = get_str_from_ptr(e.desc);
                    let desc_categorized = get_str_from_ptr(e.desc_categorized);
                    let category_key = get_str_from_ptr(e.category_key);
                    let info_categorized = get_str_from_ptr(e.info_categorized);
                    let mut values: Vec<Values> = Vec::new();

                    for retro_value in e.values {
                        if !retro_value.label.is_null() {
                            let value = get_str_from_ptr(retro_value.value);
                            let label = get_str_from_ptr(retro_value.label);

                            values.push(Values { label, value });
                        }
                    }

                    options_manager.opts.push(Options {
                        key,
                        default_value,
                        info,
                        desc,
                        category_key,
                        desc_categorized,
                        info_categorized,
                        values,
                    })
                } else {
                    break;
                }
            }
        }
    }

    options_manager
}
