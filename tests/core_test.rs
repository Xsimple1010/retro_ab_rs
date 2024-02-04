use retro_ab::*;

mod common;

#[test]
fn core_implement_tests() {
    let ctx = common::core::setup();

    match &ctx {
        Ok(ctx) => {
            assert_eq!(
                *ctx.core.language.lock().unwrap(),
                retro_language::RETRO_LANGUAGE_ENGLISH
            );

            assert_eq!(
                *ctx.core.video.pixel_format.lock().unwrap(),
                retro_pixel_format::RETRO_PIXEL_FORMAT_UNKNOWN
            );

            core::de_init(ctx.clone());
        }
        _ => panic!("O contexto não foi criado"),
    }

    // println!("{:?}", get_num_context());
}
