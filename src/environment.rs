use crate::{
    av_info,
    binding::{
        binding_libretro::{
            retro_controller_info, retro_core_option_display, retro_core_options_v2_intl,
            retro_game_geometry, retro_language, retro_log_level, retro_pixel_format,
            retro_subsystem_info, retro_variable, RETRO_ENVIRONMENT_GET_AUDIO_VIDEO_ENABLE,
            RETRO_ENVIRONMENT_GET_CORE_OPTIONS_VERSION, RETRO_ENVIRONMENT_GET_INPUT_BITMASKS,
            RETRO_ENVIRONMENT_GET_LANGUAGE, RETRO_ENVIRONMENT_GET_LOG_INTERFACE,
            RETRO_ENVIRONMENT_GET_SAVE_DIRECTORY, RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY,
            RETRO_ENVIRONMENT_GET_VARIABLE, RETRO_ENVIRONMENT_GET_VARIABLE_UPDATE,
            RETRO_ENVIRONMENT_SET_CONTROLLER_INFO, RETRO_ENVIRONMENT_SET_CORE_OPTIONS_DISPLAY,
            RETRO_ENVIRONMENT_SET_CORE_OPTIONS_UPDATE_DISPLAY_CALLBACK,
            RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2_INTL, RETRO_ENVIRONMENT_SET_GEOMETRY,
            RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS, RETRO_ENVIRONMENT_SET_PERFORMANCE_LEVEL,
            RETRO_ENVIRONMENT_SET_PIXEL_FORMAT, RETRO_ENVIRONMENT_SET_SUBSYSTEM_INFO,
            RETRO_ENVIRONMENT_SET_SUPPORT_ACHIEVEMENTS, RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME,
            RETRO_ENVIRONMENT_SET_VARIABLES,
        },
        binding_log_interface,
    },
    constants::{self, MAX_CORE_SUBSYSTEM_INFO},
    controller_info,
    managers::option_manager,
    retro_context::RetroContext,
    system,
    tools::{
        self,
        ffi_tools::{get_str_from_ptr, make_c_string},
    },
};
use ::std::os::raw;
use std::{os::raw::c_void, sync::Arc};

pub struct RetroEnvCallbacks {
    pub video_refresh_callback: fn(data: *const c_void, width: u32, height: u32, pitch: usize),
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
    match &CONTEXT {
        Some(ctx) => {
            (ctx.callbacks.video_refresh_callback)(_data, _width, _height, _pitch);
        }
        None => {}
    }
}

unsafe extern "C" fn core_log(level: retro_log_level, log: *const ::std::os::raw::c_char) {
    println!("[{:?}]: {:?}", level, get_str_from_ptr(log));
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
                        ctx,
                        tools::ffi_tools::get_str_from_ptr(option.key).as_str(),
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
            println!("RETRO_ENVIRONMENT_SET_GEOMETRY -> ok");
            let raw_geometry_ptr = _data as *mut retro_game_geometry;

            if raw_geometry_ptr.is_null() {
                return false;
            }

            match &CONTEXT {
                Some(ctx) => {
                    av_info::try_set_new_geometry(&ctx, raw_geometry_ptr);
                }
                _ => return false,
            }

            return true;
        }
        RETRO_ENVIRONMENT_SET_PIXEL_FORMAT => {
            println!("RETRO_ENVIRONMENT_SET_PIXEL_FORMAT -> ok");

            match &CONTEXT {
                Some(ctx) => {
                    *ctx.core.av_info.video.pixel_format.lock().unwrap() =
                        *(_data as *mut retro_pixel_format);
                }
                None => return false,
            }
            return true;
        }
        RETRO_ENVIRONMENT_GET_VARIABLE_UPDATE => {
            println!("RETRO_ENVIRONMENT_GET_VARIABLE_UPDATE -> ok");

            match &CONTEXT {
                Some(ctx) => {
                    if !ctx.options.opts.lock().unwrap().is_empty() {
                        *(_data as *mut bool) = *ctx.options.updated.lock().unwrap()
                    } else {
                        *(_data as *mut bool) = false;
                    }
                }
                _ => return false,
            }

            return true;
        }
        RETRO_ENVIRONMENT_SET_VARIABLES => {
            println!("RETRO_ENVIRONMENT_SET_VARIABLES");
        }
        RETRO_ENVIRONMENT_GET_VARIABLE => {
            println!("RETRO_ENVIRONMENT_GET_VARIABLE -> ok");

            let raw_variable = _data as *const retro_variable;

            if raw_variable.is_null() {
                return true;
            }

            binding_log_interface::set_variable_value_as_null(_data);

            match &CONTEXT {
                Some(ctx) => {
                    if ctx.options.opts.lock().unwrap().is_empty() {
                        return true;
                    }

                    let raw_variable = *(_data as *const retro_variable);
                    let key = get_str_from_ptr(raw_variable.key);

                    for opt in &*ctx.options.opts.lock().unwrap() {
                        if opt.key.lock().unwrap().eq(&key) {
                            let new_value = make_c_string(&*opt.selected.lock().unwrap()).unwrap();

                            let result = binding_log_interface::set_new_value_variable(
                                _data,
                                new_value.as_ptr(),
                            );

                            *ctx.options.updated.lock().unwrap() = false;

                            return result;
                        }
                    }
                }
                _ => return true,
            }
        }
        RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS => {
            println!("RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS");
        }
        RETRO_ENVIRONMENT_GET_LOG_INTERFACE => {
            println!("RETRO_ENVIRONMENT_GET_LOG_INTERFACE -> ok");

            binding_log_interface::configure_log_interface(Some(core_log), _data);

            return true;
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
            println!("RETRO_ENVIRONMENT_GET_INPUT_BITMASKS -> ok");
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
// #[cfg(test)]
#[cfg(test)]
mod test_environment {
    use std::ffi::c_void;

    use crate::{
        binding::binding_libretro::{
            retro_pixel_format, RETRO_ENVIRONMENT_GET_INPUT_BITMASKS,
            RETRO_ENVIRONMENT_SET_PIXEL_FORMAT,
        },
        environment::{configure, CONTEXT},
        test_tools,
    };

    use super::core_environment;

    fn cfg_test() {
        let ctx = test_tools::context::get_context(test_tools::core::get_raw().unwrap());
        configure(ctx);
    }

    #[test]
    fn input_bitmasks() {
        let my_bool = true;
        let data = &my_bool as *const bool as *mut c_void;

        let result = unsafe { core_environment(RETRO_ENVIRONMENT_GET_INPUT_BITMASKS, data) };

        assert_eq!(result, true);

        assert_eq!(my_bool, true);
    }

    #[test]
    fn pixel_format() {
        cfg_test();
        let pixel = retro_pixel_format::RETRO_PIXEL_FORMAT_RGB565;
        let data = &pixel as *const retro_pixel_format as *mut std::ffi::c_void;

        let result = unsafe { core_environment(RETRO_ENVIRONMENT_SET_PIXEL_FORMAT, data) };

        assert_eq!(
            result, true,
            "returno inesperado: valor desejado -> true; valor recebido -> {:?}",
            result,
        );

        unsafe {
            match &CONTEXT {
                Some(ctx) => assert_eq!(
                    *ctx.core.av_info.video.pixel_format.lock().unwrap(),
                    pixel,
                    "returno inesperado: valor desejado -> {:?}; valor recebido -> {:?}",
                    pixel,
                    *ctx.core.av_info.video.pixel_format.lock().unwrap()
                ),
                _ => panic!("contexto nao foi encontrado"),
            }
        }
    }
}
