use retro_ab::*;

mod common;

#[test]
fn context() {
    let ctx = common::setup();

    match &ctx {
        Ok(ctx) => {
            assert_eq!(
                *ctx.core.language.lock().unwrap(),
                retro_language::RETRO_LANGUAGE_ENGLISH
            );

            assert_eq!(
                *ctx.core.video.pixel_format.lock().unwrap(),
                retro_pixel_format::RETRO_PIXEL_FORMAT_UNKNOWN
            )
        }
        _ => {}
    }

    println!("{:?}", get_num_context());
}
