use super::{binding_libretro::*, core::CoreWrapper};
use ::std::os::raw;

struct _Environment {
    core: Box<CoreWrapper>,
}

static mut ENVIRONMENT: Option<_Environment> = None;

pub fn configure(core_wrapper: CoreWrapper) -> Result<&'static CoreWrapper, String> {
    let core_wrapper = Box::new(core_wrapper);

    unsafe {
        ENVIRONMENT = Some(_Environment { core: core_wrapper });

        match &ENVIRONMENT {
            Some(env) => Ok(env.core.as_ref()),
            None => Err(String::from("value")),
        }
    }
}

pub unsafe extern "C" fn core_environment(
    cmd: ::std::os::raw::c_uint,
    data: *mut raw::c_void,
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
