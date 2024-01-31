use crate::{
    binding_libretro::{
        retro_core_option_v2_definition, retro_core_options_v2, retro_core_options_v2_intl,
    },
    ffi_tools,
};
use std::{ffi::c_void, path::PathBuf};
pub struct Values {
    pub value: String,
    pub label: String,
}

#[derive(Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub enum OptionVersion {
    Legacy,
    V1Intl,
    V1,
    V2Intl,
    V2,
}

pub struct Options {
    pub key: String,
    pub visibility: bool,
    pub desc: String,
    pub desc_categorized: String,
    pub info: String,
    pub info_categorized: String,
    pub category_key: String,
    pub values: Vec<Values>,
    pub default_value: String,
}

pub struct OptionManager {
    pub version: OptionVersion,
    pub file_path: PathBuf,
    pub opts: Vec<Options>,
    pub origin_ptr: *mut c_void,
}

impl OptionManager {
    pub fn new(file_path: PathBuf) -> OptionManager {
        let expect_value = "".to_owned();
        let origin_ptr = &expect_value as *const String as *mut c_void;

        OptionManager {
            version: OptionVersion::V2,
            file_path,
            opts: Vec::new(),
            origin_ptr,
        }
    }

    pub fn update(&self, key: &str, value: &str) {
        match self.version {
            OptionVersion::Legacy => {}
            OptionVersion::V1Intl => {}
            OptionVersion::V1 => {}
            OptionVersion::V2Intl => self.update_value_v2_intl(key, value),
            OptionVersion::V2 => {}
        }
    }

    pub fn change_visibility(&mut self, key: String, visibility: bool) {
        for opt in &mut self.opts {
            if opt.key == key {
                opt.visibility = visibility;
            }
        }
    }

    fn update_value_v2_intl(&self, _key: &str, _value: &str) {
        let _op = unsafe { *(self.origin_ptr as *mut retro_core_options_v2_intl) };
    }
}

//===============================================
//=================v2_intl=======================
//===============================================
fn get_v2_intl_definitions(
    definitions: *mut retro_core_option_v2_definition,
    options_manager: &mut OptionManager,
) {
    let definitions = unsafe { *(definitions as *mut [retro_core_option_v2_definition; 90]) };

    for definition in definitions {
        if !definition.key.is_null() {
            let key = ffi_tools::get_str_from_ptr(definition.key);
            let default_value = ffi_tools::get_str_from_ptr(definition.default_value);
            let info = ffi_tools::get_str_from_ptr(definition.info);
            let desc = ffi_tools::get_str_from_ptr(definition.desc);
            let desc_categorized = ffi_tools::get_str_from_ptr(definition.desc_categorized);
            let category_key = ffi_tools::get_str_from_ptr(definition.category_key);
            let info_categorized = ffi_tools::get_str_from_ptr(definition.info_categorized);
            let mut values: Vec<Values> = Vec::new();

            for retro_value in definition.values {
                if !retro_value.label.is_null() {
                    let value = ffi_tools::get_str_from_ptr(retro_value.value);
                    let label = ffi_tools::get_str_from_ptr(retro_value.label);

                    values.push(Values { label, value });
                }
            }

            options_manager.opts.push(Options {
                key,
                visibility: true,
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

pub fn convert_option_v2_intl(
    option_intl_v2: retro_core_options_v2_intl,
    options_manager: &mut OptionManager,
) {
    options_manager.version = OptionVersion::V2Intl;

    unsafe {
        if option_intl_v2.local.is_null() {
            let us: retro_core_options_v2 = *(option_intl_v2.us);
            get_v2_intl_definitions(us.definitions, options_manager);
        } else {
            let local: retro_core_options_v2 = *(option_intl_v2.local);
            get_v2_intl_definitions(local.definitions, options_manager);
        }
    }
}
//===============================================
