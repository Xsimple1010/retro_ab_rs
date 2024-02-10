use crate::{
    constants::{self, MAX_CORE_SUBSYSTEM_INFO},
    controller_info,
    libretro::binding_libretro::{
        retro_controller_info, retro_core_option_display, retro_core_options_v2_intl,
        retro_language, retro_pixel_format, retro_subsystem_info,
        RETRO_ENVIRONMENT_GET_AUDIO_VIDEO_ENABLE, RETRO_ENVIRONMENT_GET_CORE_OPTIONS_VERSION,
        RETRO_ENVIRONMENT_GET_INPUT_BITMASKS, RETRO_ENVIRONMENT_GET_LANGUAGE,
        RETRO_ENVIRONMENT_GET_LOG_INTERFACE, RETRO_ENVIRONMENT_GET_SAVE_DIRECTORY,
        RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY, RETRO_ENVIRONMENT_GET_VARIABLE,
        RETRO_ENVIRONMENT_GET_VARIABLE_UPDATE, RETRO_ENVIRONMENT_SET_CONTROLLER_INFO,
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_DISPLAY,
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_UPDATE_DISPLAY_CALLBACK,
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2_INTL, RETRO_ENVIRONMENT_SET_GEOMETRY,
        RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS, RETRO_ENVIRONMENT_SET_PERFORMANCE_LEVEL,
        RETRO_ENVIRONMENT_SET_PIXEL_FORMAT, RETRO_ENVIRONMENT_SET_SUBSYSTEM_INFO,
        RETRO_ENVIRONMENT_SET_SUPPORT_ACHIEVEMENTS, RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME,
        RETRO_ENVIRONMENT_SET_VARIABLES,
    },
    managers::option_manager,
    retro_context::RetroContext,
    system, tools,
};
use ::std::os::raw;
use std::{os::raw::c_void, sync::Arc};

pub struct RetroEnvCallbacks {
    pub video_refresh_callback: fn(data: *const c_void, width: i32, height: i32, pitch: usize),
    pub audio_sample_callback: fn(left: i16, right: i16),
    pub audio_sample_batch_callback: fn(data: *const i16, frames: usize) -> usize,
    pub input_poll_callback: fn(),
    pub input_state_callback: fn(port: i16, device: i16, index: i16, id: i16) -> i16,
}

static mut CONTEXT: Option<Arc<RetroContext>> = None;

pub fn configure(context: Arc<RetroContext>) {
    unsafe {
        CONTEXT = Some(context);
    }
}

pub fn delete_local_ctx() {
    unsafe {
        CONTEXT = None;
    }
}

pub unsafe extern "C" fn audio_sample_callback(left: i16, right: i16) {
    if let Some(ctx) = &CONTEXT {
        (ctx.callbacks.audio_sample_callback)(left, right)
    }
}

pub unsafe extern "C" fn audio_sample_batch_callback(_data: *const i16, frames: usize) -> usize {
    if let Some(ctx) = &CONTEXT {
        (ctx.callbacks.audio_sample_batch_callback)(_data, frames)
    } else {
        0
    }
}

pub unsafe extern "C" fn input_poll_callback() {
    if let Some(ctx) = &CONTEXT {
        (ctx.callbacks.input_poll_callback)()
    }
}

pub unsafe extern "C" fn input_state_callback(
    _port: raw::c_uint,
    _device: raw::c_uint,
    _index: raw::c_uint,
    _id: raw::c_uint,
) -> i16 {
    match &CONTEXT {
        Some(ctx) => (ctx.callbacks.input_state_callback)(
            _port as i16,
            _device as i16,
            _index as i16,
            _id as i16,
        ),
        None => 0,
    }
}

pub unsafe extern "C" fn video_refresh_callback(
    _data: *const raw::c_void,
    _width: raw::c_uint,
    _height: raw::c_uint,
    _pitch: usize,
) {
}

pub unsafe extern "C" fn core_environment(
    cmd: ::std::os::raw::c_uint,
    _data: *mut raw::c_void,
) -> bool {
    match cmd {
        RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME => {
            println!("RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME -> ok");

            match &CONTEXT {
                Some(ctx) => {
                    *ctx.core.support_no_game.lock().unwrap() = *(_data as *mut bool);
                }
                None => return false,
            }

            return true;
        }
        RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY => {
            println!("RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY");
        }
        RETRO_ENVIRONMENT_GET_SAVE_DIRECTORY => {
            println!("RETRO_ENVIRONMENT_GET_SAVE_DIRECTORY");
        }
        RETRO_ENVIRONMENT_SET_SUPPORT_ACHIEVEMENTS => {
            println!("RETRO_ENVIRONMENT_SET_SUPPORT_ACHIEVEMENTS");
        }
        RETRO_ENVIRONMENT_SET_PERFORMANCE_LEVEL => {
            println!("RETRO_ENVIRONMENT_SET_PERFORMANCE_LEVEL");
        }
        RETRO_ENVIRONMENT_GET_CORE_OPTIONS_VERSION => {
            println!("RETRO_ENVIRONMENT_GET_CORE_OPTIONS_VERSION -> ok");
            *(_data as *mut u32) = 2;
            return true;
        }
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2_INTL => {
            println!("RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2_INTL -> ok");

            match &CONTEXT {
                Some(ctx) => {
                    let option_intl_v2 = *(_data as *mut retro_core_options_v2_intl);

                    option_manager::convert_option_v2_intl(option_intl_v2, ctx);
                    option_manager::try_reload_pref_option(ctx);
                }
                _ => return false,
            }

            return true;
        }
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_DISPLAY => {
            println!("RETRO_ENVIRONMENT_SET_CORE_OPTIONS_DISPLAY -> ok");

            match &CONTEXT {
                Some(ctx) => {
                    let option = *(_data as *mut retro_core_option_display);

                    option_manager::change_visibility(
                        Arc::clone(ctx),
                        tools::ffi_tools::get_str_from_ptr(option.key),
                        option.visible,
                    )
                }
                _ => return false,
            }

            return true;
        }
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_UPDATE_DISPLAY_CALLBACK => {
            println!("RETRO_ENVIRONMENT_SET_CORE_OPTIONS_UPDATE_DISPLAY_CALLBACK");
        }
        RETRO_ENVIRONMENT_GET_LANGUAGE => {
            println!("RETRO_ENVIRONMENT_GET_LANGUAGE -> ok");
            *(_data as *mut retro_language) = retro_language::RETRO_LANGUAGE_ENGLISH;
            match &CONTEXT {
                Some(ctx) => {
                    *ctx.core.language.lock().unwrap() = *(_data as *mut retro_language);
                }
                None => return false,
            }
            return true;
        }
        RETRO_ENVIRONMENT_SET_GEOMETRY => {
            println!("RETRO_ENVIRONMENT_SET_GEOMETRY");
        }
        RETRO_ENVIRONMENT_SET_PIXEL_FORMAT => {
            println!("RETRO_ENVIRONMENT_SET_PIXEL_FORMAT -> ok");

            match &CONTEXT {
                Some(ctx) => {
                    *ctx.core.video.pixel_format.lock().unwrap() =
                        *(_data as *mut retro_pixel_format);
                }
                None => return false,
            }
            return true;
        }
        RETRO_ENVIRONMENT_GET_VARIABLE_UPDATE => {
            println!("RETRO_ENVIRONMENT_GET_VARIABLE_UPDATE");
        }
        RETRO_ENVIRONMENT_SET_VARIABLES => {
            println!("RETRO_ENVIRONMENT_SET_VARIABLES");
        }
        RETRO_ENVIRONMENT_GET_VARIABLE => {
            println!("RETRO_ENVIRONMENT_GET_VARIABLE ");
        }
        RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS => {
            println!("RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS");
        }
        RETRO_ENVIRONMENT_GET_LOG_INTERFACE => {
            println!("RETRO_ENVIRONMENT_GET_LOG_INTERFACE");

            //TODO: isso esta fazendo muita falta preciso implementa isso o mais rápido possível
            //o rust nao deixa faze unsafe extern "C" fn(level: retro_log_level, fmt: *const ::std::os::raw::c_char, ...)
            //                                                                                                        |provavelmente por causa dessa merda aqui!
        }
        RETRO_ENVIRONMENT_SET_SUBSYSTEM_INFO => {
            println!("RETRO_ENVIRONMENT_SET_SUBSYSTEM_INFO -> OK");

            match &CONTEXT {
                Some(ctx) => {
                    let raw_subsystem =
                        *(_data as *mut [retro_subsystem_info; MAX_CORE_SUBSYSTEM_INFO]);

                    system::get_subsystem(ctx, raw_subsystem);
                }
                None => return false,
            }

            return true;
        }
        RETRO_ENVIRONMENT_GET_INPUT_BITMASKS => {
            println!("RETRO_ENVIRONMENT_GET_INPUT_BITMASKS");
            return true;
        }
        RETRO_ENVIRONMENT_SET_CONTROLLER_INFO => {
            println!("RETRO_ENVIRONMENT_SET_CONTROLLER_INFO -> ok");

            match &CONTEXT {
                Some(ctx) => {
                    let raw_ctr_infos = *(_data
                        as *mut [retro_controller_info; constants::MAX_CORE_CONTROLLER_INFO_TYPES]);

                    ctx.core.system.ports.lock().unwrap().clear();

                    for raw_ctr_info in raw_ctr_infos {
                        if raw_ctr_info.num_types != 0 {
                            let controller_info =
                                controller_info::get_controller_info(raw_ctr_info);

                            ctx.core.system.ports.lock().unwrap().push(controller_info);
                        } else {
                            break;
                        }
                    }
                }
                _ => return false,
            }

            return true;
        }

        RETRO_ENVIRONMENT_GET_AUDIO_VIDEO_ENABLE => {
            println!("RETRO_ENVIRONMENT_GET_AUDIO_VIDEO_ENABLE");

            *(_data as *mut u32) = 1 << 0 | 1 << 1;

            return true;
        }
        _ => {
            println!("{:?}", cmd);

            return false;
        }
    }
    false
}

//TODO: novos teste para "fn core_environment"
