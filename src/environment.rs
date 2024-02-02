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
    ffi_tools, option_manager,
    retro_context::RetroContext,
};
use ::std::os::raw;
use std::rc::Rc;

static mut CONTEXT: Option<Rc<RetroContext>> = None;

pub fn configure(context: Rc<RetroContext>) {
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
                    *ctx.core.support_no_game.borrow_mut() = *(_data as *mut bool);
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

                    option_manager::convert_option_v2_intl(options_v2, Rc::clone(ctx));
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
                        Rc::clone(ctx),
                        ffi_tools::get_str_from_ptr(option.key),
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
                    *ctx.core.language.borrow_mut() = *(_data as *mut retro_language);
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
                    *ctx.core.video.pixel_format.borrow_mut() = *(_data as *mut retro_pixel_format);
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
                    *ctx.core.use_subsystem.borrow_mut() = true;
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

#[cfg(test)]
mod environment {
    use crate::core::{self, CoreCallbacks};

    use super::*;

    fn audio_sample_callback(_left: i16, _right: i16) {}

    fn audio_sample_batch_callback(_data: *const i16, _frames: usize) -> usize {
        println!("{_frames}");
        0
    }

    fn input_poll_callback() {}

    fn input_state_callback(_port: i16, _device: i16, _index: i16, _id: i16) -> i16 {
        println!("{_port} {_device}");
        0
    }

    fn video_refresh_callback(
        _data: *const ::std::os::raw::c_void,
        _width: i32,
        _height: i32,
        _pitch: usize,
    ) {
    }

    #[test]
    fn test_configure() {
        let callbacks = CoreCallbacks {
            audio_sample_batch_callback,
            audio_sample_callback,
            input_poll_callback,
            input_state_callback,
            video_refresh_callback,
        };

        let path = "cores/test.dll".to_string();
        let context = core::load(&path, callbacks);

        match context {
            Ok(ctx) => {
                configure(ctx);
            }
            _ => {}
        }
    }

    #[test]
    fn support_no_game() {
        test_configure();

        let my_bool = true;
        let data = &my_bool as *const bool as *mut std::ffi::c_void;

        unsafe {
            let result = core_environment(RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME, data);

            assert_eq!(
                result, true,
                "returno inesperado: valor desejado -> true; valor recebido -> {:?}",
                result,
            );

            match &CONTEXT {
                Some(ctx) => assert_eq!(
                    *ctx.core.support_no_game.borrow(),
                    my_bool,
                    "returno inesperado: valor desejado -> {:?}; valor recebido -> {:?}",
                    my_bool,
                    *ctx.core.support_no_game.borrow()
                ),
                _ => panic!("contexto nao foi encontrado"),
            }
        }
    }

    #[test]
    fn language() {
        test_configure();

        let language = retro_language::RETRO_LANGUAGE_PORTUGUESE_BRAZIL;
        let data = &language as *const retro_language as *mut std::ffi::c_void;

        unsafe {
            let result = core_environment(RETRO_ENVIRONMENT_GET_LANGUAGE, data);

            assert_eq!(
                result, true,
                "returno inesperado: valor desejado -> true; valor recebido -> {:?}",
                result,
            );

            match &CONTEXT {
                Some(ctx) => assert_eq!(
                    *ctx.core.language.borrow(),
                    language,
                    "returno inesperado: valor desejado -> {:?}; valor recebido -> {:?}",
                    language,
                    *ctx.core.language.borrow()
                ),
                _ => panic!("contexto nao foi encontrado"),
            }
        }
    }

    #[test]
    fn pixel_format() {
        test_configure();

        let pixel = retro_pixel_format::RETRO_PIXEL_FORMAT_RGB565;
        let data = &pixel as *const retro_pixel_format as *mut std::ffi::c_void;

        unsafe {
            let result = core_environment(RETRO_ENVIRONMENT_SET_PIXEL_FORMAT, data);

            assert_eq!(
                result, true,
                "returno inesperado: valor desejado -> true; valor recebido -> {:?}",
                result,
            );

            match &CONTEXT {
                Some(ctx) => assert_eq!(
                    *ctx.core.video.pixel_format.borrow(),
                    pixel,
                    "returno inesperado: valor desejado -> {:?}; valor recebido -> {:?}",
                    pixel,
                    *ctx.core.video.pixel_format.borrow()
                ),
                _ => panic!("contexto nao foi encontrado"),
            }
        }
    }

    #[test]
    fn option_version() {
        test_configure();

        let version: u32 = 0;
        let expect_value = 2;
        let data = &version as *const u32 as *mut std::ffi::c_void;

        unsafe {
            let result = core_environment(RETRO_ENVIRONMENT_GET_CORE_OPTIONS_VERSION, data);

            assert_eq!(
                result, true,
                "returno inesperado: valor desejado -> true; valor recebido -> {:?}",
                result,
            );

            assert_eq!(
                version, expect_value,
                "returno inesperado: valor desejado -> {:?}; valor recebido -> {:?}",
                expect_value, version
            );
        }
    }
}
