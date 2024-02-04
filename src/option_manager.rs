use crate::{
    binding_libretro::{
        retro_core_option_v2_category, retro_core_option_v2_definition, retro_core_options_v2,
        retro_core_options_v2_intl,
    },
    retro_context::RetroContext,
    tools::mutex_tools::get_string_mutex_from_ptr,
};
use std::{
    os::raw::c_void,
    path::PathBuf,
    sync::{Arc, Mutex},
};
pub struct Values {
    pub value: Mutex<String>,
    pub label: Mutex<String>,
}

// #[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[derive(Default)]
pub enum OptionVersion {
    Legacy,
    V1Intl,
    V1,
    V2Intl,
    #[default]
    V2,
}

pub struct Options {
    pub key: Mutex<String>,
    pub visibility: Mutex<bool>,
    pub desc: Mutex<String>,
    pub desc_categorized: Mutex<String>,
    pub info: Mutex<String>,
    pub info_categorized: Mutex<String>,
    pub category_key: Mutex<String>,
    pub values: Mutex<Vec<Values>>,
    pub default_value: Mutex<String>,
}

#[derive(Default)]
pub struct Categories {
    pub key: Mutex<String>,
    pub info: Mutex<String>,
    pub desc: Mutex<String>,
}

pub struct OptionManager {
    pub version: Mutex<OptionVersion>,
    pub file_path: Mutex<PathBuf>,
    pub categories: Mutex<Vec<Categories>>,
    pub opts: Mutex<Vec<Options>>,
    _origin_ptr: Mutex<*mut c_void>,
}

impl OptionManager {
    pub fn new(file_path: PathBuf) -> OptionManager {
        let expect_value = "".to_owned();
        let origin_ptr = &expect_value as *const String as *mut c_void;

        OptionManager {
            version: Mutex::new(OptionVersion::V2),
            categories: Mutex::new(Vec::new()),
            file_path: Mutex::new(file_path),
            opts: Mutex::new(Vec::new()),
            _origin_ptr: Mutex::new(origin_ptr),
        }
    }
}

pub fn _update(ctx: Arc<RetroContext>, key: &str, value: &str) {
    match *ctx.options.version.lock().unwrap() {
        OptionVersion::Legacy => {}
        OptionVersion::V1Intl => {}
        OptionVersion::V1 => {}
        OptionVersion::V2Intl => _update_value_v2_intl(Arc::clone(&ctx), key, value),
        OptionVersion::V2 => {}
    }
}

pub fn change_visibility(ctx: Arc<RetroContext>, key: String, visibility: bool) {
    for opt in &mut *ctx.options.opts.lock().unwrap() {
        if *opt.key.lock().unwrap() == key {
            *opt.visibility.lock().unwrap() = visibility;
        }
    }
}

//===============================================
//=================v2_intl=======================
//===============================================
fn _update_value_v2_intl(ctx: Arc<RetroContext>, _key: &str, _value: &str) {
    let _origin_options =
        unsafe { *(*ctx.options._origin_ptr.lock().unwrap() as *mut retro_core_options_v2_intl) };
}

fn get_v2_intl_category(categories: *mut retro_core_option_v2_category, ctx: &Arc<RetroContext>) {
    let categories = unsafe { *(categories as *mut [retro_core_option_v2_category; 90]) };

    for category in categories {
        if !category.key.is_null() {
            let key = get_string_mutex_from_ptr(category.key);
            let info = get_string_mutex_from_ptr(category.info);
            let desc = get_string_mutex_from_ptr(category.desc);

            ctx.options
                .categories
                .lock()
                .unwrap()
                .push(Categories { key, desc, info });
        } else {
            break;
        }
    }
}

fn get_v2_intl_definitions(
    definitions: *mut retro_core_option_v2_definition,
    ctx: &Arc<RetroContext>,
) {
    let definitions = unsafe { *(definitions as *mut [retro_core_option_v2_definition; 90]) };

    for definition in definitions {
        if !definition.key.is_null() {
            let key = get_string_mutex_from_ptr(definition.key);
            let default_value = get_string_mutex_from_ptr(definition.default_value);
            let info = get_string_mutex_from_ptr(definition.info);
            let desc = get_string_mutex_from_ptr(definition.desc);
            let desc_categorized = get_string_mutex_from_ptr(definition.desc_categorized);
            let category_key = get_string_mutex_from_ptr(definition.category_key);
            let info_categorized = get_string_mutex_from_ptr(definition.info_categorized);
            let values = Mutex::new(Vec::new());

            for retro_value in definition.values {
                if !retro_value.label.is_null() {
                    let value = get_string_mutex_from_ptr(retro_value.value);
                    let label = get_string_mutex_from_ptr(retro_value.label);

                    values.lock().unwrap().push(Values { label, value });
                }
            }

            ctx.options.opts.lock().unwrap().push(Options {
                key,
                visibility: Mutex::new(true),
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
    origin_data: *mut c_void,
    ctx: Arc<RetroContext>,
) {
    *ctx.options.version.lock().unwrap() = OptionVersion::V2Intl;
    *ctx.options._origin_ptr.lock().unwrap() = origin_data;

    unsafe {
        if option_intl_v2.local.is_null() {
            let us: retro_core_options_v2 = *(option_intl_v2.us);
            get_v2_intl_definitions(us.definitions, &ctx);
            get_v2_intl_category(us.categories, &ctx);
        } else {
            let local: retro_core_options_v2 = *(option_intl_v2.local);
            get_v2_intl_definitions(local.definitions, &ctx);
        }
    }
}
//===============================================
