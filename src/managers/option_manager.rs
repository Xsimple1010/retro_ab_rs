use crate::{
    binding::binding_libretro::{
        retro_core_option_v2_category, retro_core_option_v2_definition, retro_core_options_v2,
        retro_core_options_v2_intl,
    },
    constants,
    retro_context::RetroContext,
    tools::mutex_tools::get_string_mutex_from_ptr,
};
use std::{
    fs::File,
    io::{Read, Write},
    path::PathBuf,
    sync::{Arc, Mutex},
};

pub struct Values {
    pub value: Mutex<String>,
    pub label: Mutex<String>,
}

#[derive(Default)]
pub struct Options {
    pub key: Mutex<String>,
    pub visibility: Mutex<bool>,
    pub selected: Mutex<String>,
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
    pub file_path: Mutex<PathBuf>,
    pub categories: Mutex<Vec<Categories>>,
    pub updated: Mutex<bool>,
    pub opts: Mutex<Vec<Options>>,
}

impl OptionManager {
    pub fn new(file_path: PathBuf) -> OptionManager {
        OptionManager {
            updated: Mutex::new(true),
            categories: Mutex::new(Vec::new()),
            file_path: Mutex::new(file_path),
            opts: Mutex::new(Vec::new()),
        }
    }
}

pub fn update(ctx: Arc<RetroContext>, key: &str, value: &str) {
    for opt in &*ctx.options.opts.lock().unwrap() {
        if opt.key.lock().unwrap().eq(key) {
            *opt.key.lock().unwrap() = key.to_string();
            *opt.selected.lock().unwrap() = value.to_string();

            *ctx.options.updated.lock().unwrap() = true;
        }
    }

    write_all_options_in_file(Arc::clone(&ctx));
}

pub fn change_visibility(ctx: Arc<RetroContext>, key: String, visibility: bool) {
    for opt in &mut *ctx.options.opts.lock().unwrap() {
        if *opt.key.lock().unwrap() == key {
            *opt.visibility.lock().unwrap() = visibility;
        }
    }
}

fn write_all_options_in_file(ctx: Arc<RetroContext>) {
    let file_path = ctx.options.file_path.lock().unwrap().clone();
    let mut file = File::create(file_path.clone()).unwrap();

    for opt in &*ctx.options.opts.lock().unwrap() {
        let key = opt.key.lock().unwrap().clone();
        let selected = opt.selected.lock().unwrap().clone();

        let buf = key + "=" + &selected + "\n";

        let _ = file.write(buf.as_bytes());
    }
}

fn load_all_option_in_file(ctx: Arc<RetroContext>) {
    let file_path = ctx.options.file_path.lock().unwrap().clone();

    let mut file = File::open(file_path).unwrap();

    let mut buf = String::new();
    file.read_to_string(&mut buf).unwrap();

    let lines: Vec<&str> = buf.split('\n').collect();

    for line in &lines {
        if line.is_empty() {
            return;
        }

        let values: Vec<&str> = line.split('=').collect();

        let key = values.first().unwrap();
        let value = values.get(1).unwrap();

        for opt in &*ctx.options.opts.lock().unwrap() {
            if opt.key.lock().unwrap().eq(key) {
                *opt.key.lock().unwrap() = key.to_string();
                *opt.selected.lock().unwrap() = value.to_string();
            }
        }
    }
}

//TODO: adiciona um meio do usuário saber se ocorrer um erro ao tentar salva ou ler o arquivo
pub fn try_reload_pref_option(ctx: &Arc<RetroContext>) {
    let file_path = ctx.options.file_path.lock().unwrap().clone();

    //se o arquivo ainda nao existe apenas
    //crie um novo arquivo e salve a configuração padrão do núcleo
    if !file_path.exists() {
        write_all_options_in_file(Arc::clone(ctx));
    } else {
        load_all_option_in_file(Arc::clone(ctx))
    }
}

//===============================================
//=================v2_intl=======================
//===============================================

fn get_v2_intl_category(categories: *mut retro_core_option_v2_category, ctx: &Arc<RetroContext>) {
    let categories = unsafe {
        *(categories as *mut [retro_core_option_v2_category; constants::MAX_CORE_OPTIONS])
    };

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
            let selected = get_string_mutex_from_ptr(definition.default_value);
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
                selected,
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

pub fn convert_option_v2_intl(option_intl_v2: retro_core_options_v2_intl, ctx: &Arc<RetroContext>) {
    unsafe {
        if option_intl_v2.local.is_null() {
            let us: retro_core_options_v2 = *(option_intl_v2.us);
            get_v2_intl_definitions(us.definitions, ctx);
            get_v2_intl_category(us.categories, ctx);
        } else {
            let local: retro_core_options_v2 = *(option_intl_v2.local);
            get_v2_intl_definitions(local.definitions, ctx);
            get_v2_intl_category(local.categories, ctx);
        }
    }
}
//===============================================
