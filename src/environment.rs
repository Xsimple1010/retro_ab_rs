use super::core::{Context, CoreCallbacks, CoreWrapper};
use crate::binding_libretro::{
    retro_language, retro_pixel_format, RETRO_ENVIRONMENT_GET_CORE_OPTIONS_VERSION,
    RETRO_ENVIRONMENT_GET_LANGUAGE, RETRO_ENVIRONMENT_GET_LOG_INTERFACE,
    RETRO_ENVIRONMENT_SET_CONTROLLER_INFO,
    RETRO_ENVIRONMENT_SET_CORE_OPTIONS_UPDATE_DISPLAY_CALLBACK,
    RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2_INTL, RETRO_ENVIRONMENT_SET_GEOMETRY,
    RETRO_ENVIRONMENT_SET_INPUT_DESCRIPTORS, RETRO_ENVIRONMENT_SET_PIXEL_FORMAT,
    RETRO_ENVIRONMENT_SET_SUBSYSTEM_INFO, RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME,
    RETRO_ENVIRONMENT_SET_VARIABLES,
};
use ::std::os::raw;
use std::cell::RefCell;

static mut CONTEXT: Option<Context> = None;

pub fn configure(
    core_wrapper: CoreWrapper,
    callback: CoreCallbacks,
) -> Result<&'static Context, String> {
    unsafe {
        CONTEXT = Some(Context {
            core: RefCell::new(core_wrapper),
            callbacks: RefCell::new(callback),
        });

        match &CONTEXT {
            Some(ctx) => Ok(ctx),
            None => Err(String::from("value")),
        }
    }
}

pub unsafe extern "C" fn audio_sample_callback(left: i16, right: i16) {
    match &CONTEXT {
        Some(ctx) => (ctx.callbacks.borrow().audio_sample_callback)(left, right),
        None => {}
    }
}

pub unsafe extern "C" fn audio_sample_batch_callback(_data: *const i16, frames: usize) -> usize {
    match &CONTEXT {
        Some(ctx) => (ctx.callbacks.borrow().audio_sample_batch_callback)(_data, frames),
        None => frames,
    }
}

pub unsafe extern "C" fn input_poll_callback() {
    match &CONTEXT {
        Some(ctx) => (ctx.callbacks.borrow().input_poll_callback)(),
        None => {}
    }
}

pub unsafe extern "C" fn input_state_callback(
    _port: raw::c_uint,
    _device: raw::c_uint,
    _index: raw::c_uint,
    _id: raw::c_uint,
) -> i16 {
    match &CONTEXT {
        Some(ctx) => (ctx.callbacks.borrow().input_state_callback)(
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
                    ctx.core.borrow_mut().support_no_game = *(_data as *mut bool);
                }
                None => {}
            }

            return true;
        }
        RETRO_ENVIRONMENT_GET_CORE_OPTIONS_VERSION => {
            println!("RETRO_ENVIRONMENT_GET_CORE_OPTIONS_VERSION -> ok");
            *(_data as *mut u32) = 2;
            return true;
        }
        RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2_INTL => {
            println!("RETRO_ENVIRONMENT_SET_CORE_OPTIONS_V2_INTL");
        }
        RETRO_ENVIRONMENT_GET_LANGUAGE => {
            println!("RETRO_ENVIRONMENT_GET_LANGUAGE -> ok");
            *(_data as *mut retro_language) = retro_language::RETRO_LANGUAGE_ENGLISH;
            match &CONTEXT {
                Some(ctx) => {
                    ctx.core.borrow_mut().language = *(_data as *mut retro_language);
                }
                None => {}
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
                    ctx.core.borrow_mut().video.pixel_format = *(_data as *mut retro_pixel_format);
                }
                None => {}
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
                    ctx.core.borrow_mut().use_subsystem = true;
                }
                None => {}
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
        let core_wrapper = CoreWrapper::new();

        let callbacks = CoreCallbacks {
            audio_sample_batch_callback,
            audio_sample_callback,
            input_poll_callback,
            input_state_callback,
            video_refresh_callback,
        };

        configure(core_wrapper, callbacks).unwrap();

        //todo: testar o contexto
        unsafe {
            match &CONTEXT {
                Some(_ctx) => {
                    // let callbacks = CoreCallbacks {
                    //     audio_sample_batch_callback,
                    //     audio_sample_callback,
                    //     input_poll_callback,
                    //     input_state_callback,
                    //     video_refresh_callback,
                    // };

                    // assert_eq!(ctx.core.borrow(), CoreWrapper::new())
                }
                _ => {}
            }
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
                    ctx.core.borrow().support_no_game,
                    my_bool,
                    "returno inesperado: valor desejado -> {:?}; valor recebido -> {:?}",
                    my_bool,
                    ctx.core.borrow().support_no_game
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
                    ctx.core.borrow().language,
                    language,
                    "returno inesperado: valor desejado -> {:?}; valor recebido -> {:?}",
                    language,
                    ctx.core.borrow().language
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
                    ctx.core.borrow().video.pixel_format,
                    pixel,
                    "returno inesperado: valor desejado -> {:?}; valor recebido -> {:?}",
                    pixel,
                    ctx.core.borrow().video.pixel_format
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
