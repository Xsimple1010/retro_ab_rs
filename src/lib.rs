pub mod args_manager;
pub mod core;

mod binding_libretro;
mod environment;
mod ffi_tools;
mod game_tools;
mod option_manager;

#[cfg(test)]
mod lib_fns {
    use crate::*;

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

    static mut CONTEXT: Option<&'static core::Context> = None;

    #[test]
    fn load_core() {
        let callbacks = core::CoreCallbacks {
            audio_sample_batch_callback,
            audio_sample_callback,
            input_poll_callback,
            input_state_callback,
            video_refresh_callback,
        };

        let path = "cores/test.dll".to_string();

        let res = core::load(&path, callbacks);

        match res {
            Ok(ctx) => unsafe {
                CONTEXT = Some(ctx);
            },
            Err(e) => panic!("{:?}", e),
        }
    }

    #[test]
    fn init() {
        //isso vai inicializar o contexto para realizar o teste atual
        load_core();
        //essa é a função que sará testada agora
        core::init();

        unsafe {
            match CONTEXT {
                Some(ctx) => {
                    assert_eq!(
                        ctx.core.borrow().initialized,
                        true,
                        "o CORE nao foi inicializado"
                    );
                }
                _ => panic!("O contexto nao foi encontrado"),
            }
        }
    }

    #[test]
    fn deinit() {
        //isso vai inicializar o contexto para realizar o teste atual
        load_core();
        //essa é a função que sará testada agora
        core::de_init();

        unsafe {
            match CONTEXT {
                Some(ctx) => {
                    assert_eq!(
                        ctx.core.borrow().initialized,
                        false,
                        "o CORE nao foi inicializado"
                    );
                }
                _ => panic!("O contexto nao foi encontrado"),
            }
        }
    }
}
