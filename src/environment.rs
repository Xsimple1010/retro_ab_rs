use crate::{
    binding_libretro::{
        retro_core_option_display, retro_core_options_v2_intl, retro_language, retro_pixel_format,
        RETRO_ENVIRONMENT_GET_CORE_OPTIONS_VERSION, RETRO_ENVIRONMENT_GET_LANGUAGE,
        RETRO_ENVIRONMENT_GET_LOG_INTERFACE, RETRO_ENVIRONMENT_SET_CONTROLLER_INFO,
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_DISPLAY,
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_UPDATE_DISPLAY_CALLBACK,
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2_INTL, RETRO_ENVIRONMENT_SET_GEOMETRY,
        RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS, RETRO_ENVIRONMENT_SET_PIXEL_FORMAT,
        RETRO_ENVIRONMENT_SET_SUBSYSTEM_INFO, RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME,
        RETRO_ENVIRONMENT_SET_VARIABLES,
    },
    option_manager,
    retro_context::RetroContext,
    tools,
};
use ::std::os::raw;
use std::sync::Arc;

static mut CONTEXT: Option<Arc<RetroContext>> = None;

pub fn configure(context: Arc<RetroContext>) {
    unsafe {
        CONTEXT = Some(context);
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
        RETRO_ENVIRONMENT_GET_CORE_OPTIONS_VERSION => {
            println!("RETRO_ENVIRONMENT_GET_CORE_OPTIONS_VERSION -> ok");
            *(_data as *mut u32) = 2;
            return true;
        }
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2_INTL => {
            println!("RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2_INTL -> ok");

            match &CONTEXT {
                Some(ctx) => {
                    let options_v2 = *(_data as *mut retro_core_options_v2_intl);

                    option_manager::convert_option_v2_intl(options_v2, Arc::clone(ctx));
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
        RETRO_ENVIRONMENT_SET_VARIABLES => {
            println!("RETRO_ENVIRONMENT_SET_VARIABLES");
        }
        RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS => {
            println!("RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS");
        }
        RETRO_ENVIRONMENT_GET_LOG_INTERFACE => {
            println!("RETRO_ENVIRONMENT_GET_LOG_INTERFACE");
        }
        RETRO_ENVIRONMENT_SET_SUBSYSTEM_INFO => {
            println!("RETRO_ENVIRONMENT_SET_SUBSYSTEM_INFO");

            match &CONTEXT {
                Some(ctx) => {
                    *ctx.core.use_subsystem.lock().unwrap() = true;
                }
                None => return false,
            }
        }
        RETRO_ENVIRONMENT_SET_CONTROLLER_INFO => {
            println!("RETRO_ENVIRONMENT_SET_CONTROLLER_INFO");
        }
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_UPDATE_DISPLAY_CALLBACK => {
            println!("RETRO_ENVIRONMENT_SET_CORE_OPTIONS_UPDATE_DISPLAY_CALLBACK");
        }

        _ => {
            println!("{:?}", cmd);

            return false;
        }
    }
    false
}

//TODO: novos teste para "fn core_environment"
