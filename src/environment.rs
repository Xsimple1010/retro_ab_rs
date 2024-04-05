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
    // retro_perf::{
    //     core_get_perf_counter, core_perf_log, core_perf_register, core_perf_start, core_perf_stop,
    //     get_cpu_features, get_features_get_time_usec,
    // },
    retro_sys::{
        retro_rumble_effect, retro_rumble_interface,
        RETRO_ENVIRONMENT_GET_DISK_CONTROL_INTERFACE_VERSION, RETRO_ENVIRONMENT_GET_LED_INTERFACE,
        RETRO_ENVIRONMENT_GET_MESSAGE_INTERFACE_VERSION, RETRO_ENVIRONMENT_GET_PERF_INTERFACE,
        RETRO_ENVIRONMENT_GET_RUMBLE_INTERFACE, RETRO_ENVIRONMENT_GET_VFS_INTERFACE,
        RETRO_ENVIRONMENT_SET_DISK_CONTROL_INTERFACE, RETRO_ENVIRONMENT_SET_SERIALIZATION_QUIRKS,
    },
    system,
    tools::{
        self,
        ffi_tools::{get_str_from_ptr, make_c_string},
    },
};
use ::std::os::raw;
use std::{os::raw::c_void, sync::Arc};

#[derive(Clone, Copy)]
pub struct RetroEnvCallbacks {
    pub video_refresh_callback: fn(data: *const c_void, width: u32, height: u32, pitch: usize),
    pub audio_sample_callback: fn(left: i16, right: i16),
    pub audio_sample_batch_callback: fn(data: *const i16, frames: usize) -> usize,
    pub input_poll_callback: fn(),
    pub input_state_callback: fn(port: i16, device: i16, index: i16, id: i16) -> i16,
    pub rumble_callback:
        fn(port: ::std::os::raw::c_uint, effect: retro_rumble_effect, strength: u16) -> bool,
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

unsafe extern "C" fn rumble_callback(
    port: ::std::os::raw::c_uint,
    effect: retro_rumble_effect,
    strength: u16,
) -> bool {
    match &CONTEXT {
        Some(ctx) => (ctx.callbacks.rumble_callback)(port, effect, strength),
        None => false,
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
            #[cfg(feature = "core_logs")]
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
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_GET_SYSTEM_DIRECTORY -> ok");

            match &CONTEXT {
                Some(ctx) => {
                    let sys_dir = make_c_string(&ctx.paths.system).unwrap();

                    binding_log_interface::set_directory(_data, sys_dir.as_ptr())
                }
                _ => return false,
            }

            return true;
        }
        RETRO_ENVIRONMENT_GET_SAVE_DIRECTORY => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_GET_SAVE_DIRECTORY -> ok");

            match &CONTEXT {
                Some(ctx) => {
                    let save_dir = make_c_string(&ctx.paths.save).unwrap();

                    binding_log_interface::set_directory(_data, save_dir.as_ptr())
                }
                _ => return false,
            }

            return true;
        }
        RETRO_ENVIRONMENT_SET_SUPPORT_ACHIEVEMENTS => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_SET_SUPPORT_ACHIEVEMENTS");
        }
        RETRO_ENVIRONMENT_SET_PERFORMANCE_LEVEL => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_SET_PERFORMANCE_LEVEL");
        }
        RETRO_ENVIRONMENT_GET_CORE_OPTIONS_VERSION => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_GET_CORE_OPTIONS_VERSION -> ok");
            *(_data as *mut u32) = 2;
            return true;
        }
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2_INTL => {
            #[cfg(feature = "core_logs")]
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
            #[cfg(feature = "core_logs")]
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
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_SET_CORE_OPTIONS_UPDATE_DISPLAY_CALLBACK");
        }
        RETRO_ENVIRONMENT_GET_LANGUAGE => {
            #[cfg(feature = "core_logs")]
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
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_SET_GEOMETRY -> ok");
            let raw_geometry_ptr = _data as *mut retro_game_geometry;

            if raw_geometry_ptr.is_null() {
                return false;
            }

            match &CONTEXT {
                Some(ctx) => {
                    av_info::try_set_new_geometry(ctx, raw_geometry_ptr);
                }
                _ => return false,
            }

            return true;
        }
        RETRO_ENVIRONMENT_SET_PIXEL_FORMAT => {
            #[cfg(feature = "core_logs")]
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
            #[cfg(feature = "core_logs")]
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
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_SET_VARIABLES");
        }
        RETRO_ENVIRONMENT_GET_VARIABLE => {
            #[cfg(feature = "core_logs")]
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
                            let new_value = make_c_string(&opt.selected.lock().unwrap()).unwrap();

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
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS");
        }
        RETRO_ENVIRONMENT_GET_LOG_INTERFACE => {
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_GET_LOG_INTERFACE -> ok");

            binding_log_interface::configure_log_interface(Some(core_log), _data);

            return true;
        }
        RETRO_ENVIRONMENT_SET_SUBSYSTEM_INFO => {
            #[cfg(feature = "core_logs")]
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
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_GET_INPUT_BITMASKS -> ok");
            return true;
        }
        RETRO_ENVIRONMENT_SET_CONTROLLER_INFO => {
            #[cfg(feature = "core_logs")]
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
            #[cfg(feature = "core_logs")]
            println!("RETRO_ENVIRONMENT_GET_AUDIO_VIDEO_ENABLE -> ok");

            *(_data as *mut u32) = 1 << 0 | 1 << 1;

            return true;
        }
        RETRO_ENVIRONMENT_GET_VFS_INTERFACE => {
            println!("RETRO_ENVIRONMENT_GET_VFS_INTERFACE");
        }
        RETRO_ENVIRONMENT_GET_LED_INTERFACE => {
            println!("RETRO_ENVIRONMENT_GET_LED_INTERFACE");
        }
        RETRO_ENVIRONMENT_GET_MESSAGE_INTERFACE_VERSION => {
            println!("RETRO_ENVIRONMENT_GET_MESSAGE_INTERFACE_VERSION");
        }
        RETRO_ENVIRONMENT_GET_DISK_CONTROL_INTERFACE_VERSION => {
            println!("RETRO_ENVIRONMENT_GET_DISK_CONTROL_INTERFACE_VERSION");
        }
        RETRO_ENVIRONMENT_SET_DISK_CONTROL_INTERFACE => {
            println!("RETRO_ENVIRONMENT_SET_DISK_CONTROL_INTERFACE");
        }
        RETRO_ENVIRONMENT_GET_PERF_INTERFACE => {
            println!("RETRO_ENVIRONMENT_GET_PERF_INTERFACE");

            // let mut perf = *(_data as *mut retro_perf_callback);

            // perf.get_time_usec = Some(get_features_get_time_usec);
            // perf.get_cpu_features = Some(get_cpu_features);
            // perf.get_perf_counter = Some(core_get_perf_counter);
            // perf.perf_register = Some(core_perf_register);
            // perf.perf_start = Some(core_perf_start);
            // perf.perf_stop = Some(core_perf_stop);
            // perf.perf_log = Some(core_perf_log);

            // return true;
        }
        RETRO_ENVIRONMENT_SET_SERIALIZATION_QUIRKS => {
            println!("RETRO_ENVIRONMENT_SET_SERIALIZATION_QUIRKS");
        }
        RETRO_ENVIRONMENT_GET_RUMBLE_INTERFACE => {
            println!("RETRO_ENVIRONMENT_GET_RUMBLE_INTERFACE -> ok");

            let mut rumble_raw = *(_data as *mut retro_rumble_interface);
            rumble_raw.set_rumble_state = Some(rumble_callback);

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
