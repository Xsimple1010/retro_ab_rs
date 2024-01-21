use super::{
    binding_libretro::*,
    core::{CoreCallbacks, CoreWrapper},
};
use ::std::os::raw;
use std::rc::Rc;

struct _Environment {
    core: Rc<CoreWrapper>,
    callbacks: Rc<CoreCallbacks>,
}

static mut ENVIRONMENT: Option<_Environment> = None;

pub fn configure(
    core_wrapper: CoreWrapper,
    callback: CoreCallbacks,
) -> Result<Rc<CoreWrapper>, String> {
    unsafe {
        let core = Rc::new(core_wrapper);

        ENVIRONMENT = Some(_Environment {
            core: core.clone(),
            callbacks: Rc::new(callback),
        });

        match &ENVIRONMENT {
            Some(_) => Ok(core),
            None => Err(String::from("value")),
        }
    }
}

pub unsafe extern "C" fn audio_sample_callback(left: i16, right: i16) {
    match &ENVIRONMENT {
        Some(env) => (env.callbacks.audio_sample_callback)(left, right),
        None => {}
    }
}

pub unsafe extern "C" fn audio_sample_batch_callback(_data: *const i16, frames: usize) -> usize {
    match &ENVIRONMENT {
        Some(env) => (env.callbacks.audio_sample_batch_callback)(_data, frames),
        None => frames,
    }
}

pub unsafe extern "C" fn input_poll_callback() {
    match &ENVIRONMENT {
        Some(env) => (env.callbacks.input_poll_callback)(),
        None => {}
    }
}

pub unsafe extern "C" fn input_state_callback(
    _port: raw::c_uint,
    _device: raw::c_uint,
    _index: raw::c_uint,
    _id: raw::c_uint,
) -> i16 {
    match &ENVIRONMENT {
        Some(env) => (env.callbacks.input_state_callback)(
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
    match &ENVIRONMENT {
        Some(env) => {
            println!("version is -> {:?}", env.core.version());
        }
        None => {
            println!("none version");
        }
    }

    match cmd {
        RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME => {
            println!("RETRO_ENVIRONMENT_SET_SUPPORT_NO_GAME");
        }
        RETRO_ENVIRONMENT_GET_CORE_OPTIONS_VERSION => {
            // data = 2;
            println!("RETRO_ENVIRONMENT_GET_CORE_OPTIONS_VERSION");
        }
        RETRO_ENVIRONMENT_SET_GEOMETRY => {
            println!("RETRO_ENVIRONMENT_SET_GEOMETRY");
        }
        RETRO_ENVIRONMENT_SET_PIXEL_FORMAT => {
            println!("RETRO_ENVIRONMENT_SET_PIXEL_FORMAT");
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
