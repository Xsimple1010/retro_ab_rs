use crate::{
    retro_sys::{
        retro_core_option_v2_category, retro_core_option_v2_definition, retro_core_options_v2,
        retro_core_options_v2_intl,
    },
    constants,
    tools::mutex_tools::get_string_mutex_from_ptr,
};
use std::{fs::File, io::{Read, Write}, path::PathBuf, sync::{Mutex}};
use crate::constants::CORE_OPTION_EXTENSION_FILE;

#[derive(Default, Debug)]
pub struct Values {
    pub value: Mutex<String>,
    pub label: Mutex<String>,
}

#[derive(Default, Debug)]
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

#[derive(Default, Debug)]
pub struct Categories {
    pub key: Mutex<String>,
    pub info: Mutex<String>,
    pub desc: Mutex<String>,
}

#[derive(Default, Debug)]
pub struct OptionManager {
    pub file_path: Mutex<PathBuf>,
    pub categories: Mutex<Vec<Categories>>,
    pub updated: Mutex<bool>,
    pub opts: Mutex<Vec<Options>>,
}

impl OptionManager {
    pub fn new(opt_path: &str, library_name: String) -> OptionManager {
        let file_path = PathBuf::from(opt_path)
            .join(library_name + CORE_OPTION_EXTENSION_FILE);

        OptionManager {
            updated: Mutex::new(true),
            categories: Mutex::new(Vec::new()),
            file_path: Mutex::new(file_path),
            opts: Mutex::new(Vec::new()),
        }
    }


    pub fn update_opt(&self, opt_key: &str, new_value_selected: &str) {
        self.change_value_selected(opt_key, new_value_selected);
        self.write_all_options_in_file();
    }

    pub fn change_visibility(&self, key: &str, visibility: bool) {
        for opt in &mut *self.opts.lock().unwrap() {
            if opt.key.lock().unwrap().eq(key) {
                *opt.visibility.lock().unwrap() = visibility;
            }
        }
    }

    fn write_all_options_in_file(&self) {
        let file_path = self.file_path.lock().unwrap().clone();
        let mut file = File::create(file_path.clone()).unwrap();

        for opt in &*self.opts.lock().unwrap() {
            let key = opt.key.lock().unwrap().clone();
            let selected = opt.selected.lock().unwrap().clone();

            let buf = key + "=" + &selected + "\n";

            let _ = file.write(buf.as_bytes());
        }
    }

    fn change_value_selected(&self, opt_key: &str, new_value_selected: &str) {
        for opt in &*self.opts.lock().unwrap() {
            if opt.key.lock().unwrap().eq(opt_key) {
                for v in &*opt.values.lock().unwrap() {
                    if *v.value.lock().unwrap() == new_value_selected {
                        *opt.selected.lock().unwrap() = new_value_selected.to_string();
                        *self.updated.lock().unwrap() = true;
                        break;
                    }
                }

                break;
            }
        }
    }

    fn load_all_option_in_file(&self) {
        let file_path = self.file_path.lock().unwrap().clone();

        let mut file = File::open(file_path).unwrap();

        let mut buf = String::new();
        file.read_to_string(&mut buf).unwrap();

        let lines: Vec<&str> = buf.split('\n').collect();

        for line in &lines {
            if line.is_empty() {
                return;
            }

            let values: Vec<&str> = line.split('=').collect();

            let opt_key = values.first().unwrap();
            let value_selected = values
                .get(1)
                .expect("nao foi possível recupera o valor do arquivo de opções")
                .split_ascii_whitespace()
                .next()
                .expect("nao foi possível recupera o valor do arquivo de opções");

            self.change_value_selected(opt_key, value_selected);
        }
    }

    //TODO: adiciona um meio do usuário saber se ocorrer um erro ao tentar salva ou ler o arquivo
    pub fn try_reload_pref_option(&self) {
        let file_path = self.file_path.lock().unwrap().clone();

        //se o arquivo ainda nao existe apenas
        //crie um novo arquivo e salve a configuração padrão do núcleo
        if !file_path.exists() {
            self.write_all_options_in_file();
        } else {
            self.load_all_option_in_file()
        }
    }

    //===============================================
    //=================v2_intl=======================
    //===============================================

    fn get_v2_intl_category(&self, categories: *mut retro_core_option_v2_category) {
        let categories = unsafe {
            *(categories as *mut [retro_core_option_v2_category; constants::MAX_CORE_OPTIONS])
        };

        for category in categories {
            if !category.key.is_null() {
                let key = get_string_mutex_from_ptr(category.key);
                let info = get_string_mutex_from_ptr(category.info);
                let desc = get_string_mutex_from_ptr(category.desc);

                self
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
        &self, definitions: *mut retro_core_option_v2_definition,
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

                self.opts.lock().unwrap().push(Options {
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

    pub fn convert_option_v2_intl(&self, option_intl_v2: retro_core_options_v2_intl) {
        unsafe {
            if option_intl_v2.local.is_null() {
                let us: retro_core_options_v2 = *(option_intl_v2.us);
                self.get_v2_intl_definitions(us.definitions);
                self.get_v2_intl_category(us.categories);
            } else {
                let local: retro_core_options_v2 = *(option_intl_v2.local);
                self.get_v2_intl_definitions(local.definitions);
                self.get_v2_intl_category(local.categories);
            }
        }
    }
    //===============================================
}


