use crate::{
    binding_libretro::{
        retro_core_option_v2_definition, retro_core_options_v2, retro_core_options_v2_intl,
    },
    ffi_tools::get_str_from_ptr,
    retro_context::RetroContext,
};
use std::{cell::RefCell, ffi::c_void, path::PathBuf, rc::Rc};
pub struct Values {
    pub value: RefCell<String>,
    pub label: RefCell<String>,
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
    pub key: RefCell<String>,
    pub visibility: RefCell<bool>,
    pub desc: RefCell<String>,
    pub desc_categorized: RefCell<String>,
    pub info: RefCell<String>,
    pub info_categorized: RefCell<String>,
    pub category_key: RefCell<String>,
    pub values: RefCell<Vec<Values>>,
    pub default_value: RefCell<String>,
}

pub struct OptionManager {
    pub version: RefCell<OptionVersion>,
    pub file_path: RefCell<PathBuf>,
    pub opts: RefCell<Vec<Options>>,
    pub origin_ptr: RefCell<*mut c_void>,
}

impl OptionManager {
    pub fn new(file_path: PathBuf) -> OptionManager {
        let expect_value = "".to_owned();
        let origin_ptr = &expect_value as *const String as *mut c_void;

        OptionManager {
            version: RefCell::new(OptionVersion::V2),
            file_path: RefCell::new(file_path),
            opts: RefCell::new(Vec::new()),
            origin_ptr: RefCell::new(origin_ptr),
        }
    }
}

pub fn update(ctx: Rc<RetroContext>, key: &str, value: &str) {
    match *ctx.options.version.borrow() {
        OptionVersion::Legacy => {}
        OptionVersion::V1Intl => {}
        OptionVersion::V1 => {}
        OptionVersion::V2Intl => update_value_v2_intl(key, value),
        OptionVersion::V2 => {}
    }
}

pub fn change_visibility(ctx: Rc<RetroContext>, key: String, visibility: bool) {
    for opt in &mut *ctx.options.opts.borrow_mut() {
        if *opt.key.borrow() == key {
            *opt.visibility.borrow_mut() = visibility;
        }
    }
}

fn update_value_v2_intl(_key: &str, _value: &str) {
    // let _op = unsafe { *(self.origin_ptr as *mut retro_core_options_v2_intl) };
}

//===============================================
//=================v2_intl=======================
//===============================================
fn get_v2_intl_definitions(
    definitions: *mut retro_core_option_v2_definition,
    ctx: Rc<RetroContext>,
) {
    let definitions = unsafe { *(definitions as *mut [retro_core_option_v2_definition; 90]) };

    for definition in definitions {
        if !definition.key.is_null() {
            let key = RefCell::new(get_str_from_ptr(definition.key));
            let default_value = RefCell::new(get_str_from_ptr(definition.default_value));
            let info = RefCell::new(get_str_from_ptr(definition.info));
            let desc = RefCell::new(get_str_from_ptr(definition.desc));
            let desc_categorized = RefCell::new(get_str_from_ptr(definition.desc_categorized));
            let category_key = RefCell::new(get_str_from_ptr(definition.category_key));
            let info_categorized = RefCell::new(get_str_from_ptr(definition.info_categorized));
            let values = RefCell::new(Vec::new());

            for retro_value in definition.values {
                if !retro_value.label.is_null() {
                    let value = RefCell::new(get_str_from_ptr(retro_value.value));
                    let label = RefCell::new(get_str_from_ptr(retro_value.label));

                    values.borrow_mut().push(Values { label, value });
                }
            }

            ctx.options.opts.borrow_mut().push(Options {
                key,
                visibility: RefCell::new(true),
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

pub fn convert_option_v2_intl(option_intl_v2: retro_core_options_v2_intl, ctx: Rc<RetroContext>) {
    *ctx.options.version.borrow_mut() = OptionVersion::V2Intl;

    unsafe {
        if option_intl_v2.local.is_null() {
            let us: retro_core_options_v2 = *(option_intl_v2.us);
            get_v2_intl_definitions(us.definitions, ctx);
        } else {
            let local: retro_core_options_v2 = *(option_intl_v2.local);
            get_v2_intl_definitions(local.definitions, ctx);
        }
    }
}
//===============================================
