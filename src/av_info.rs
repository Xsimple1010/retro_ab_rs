use std::sync::{Arc, Mutex};

use crate::{
    binding::binding_libretro::retro_game_geometry,
    core::{retro_pixel_format, RetroContext},
};

#[derive(Default)]
pub struct Timing {
    #[doc = "FPS of video content."]
    pub fps: Mutex<i32>,
    #[doc = "Sampling rate of audio."]
    pub sample_rate: Mutex<i32>,
}

#[derive(Default)]
pub struct Geometry {
    #[doc = "Nominal video width of game."]
    pub base_width: Mutex<u32>,

    #[doc = "Nominal video height of game."]
    pub base_height: Mutex<u32>,

    #[doc = "Maximum possible width of game."]
    pub max_width: Mutex<u32>,

    #[doc = "Maximum possible height of game."]
    pub max_height: Mutex<u32>,

    #[doc = "Nominal aspect ratio of game. If
    aspect_ratio is <= 0.0, an aspect ratio
    of base_width / base_height is assumed.
    A frontend could override this setting,
    if desired."]
    pub aspect_ratio: Mutex<f32>,
}

pub struct Video {
    pub can_dupe: Mutex<bool>,
    pub pixel_format: Mutex<retro_pixel_format>,
    pub geometry: Geometry,
}

impl Default for Video {
    fn default() -> Self {
        Video {
            can_dupe: Mutex::new(false),
            pixel_format: Mutex::new(retro_pixel_format::RETRO_PIXEL_FORMAT_UNKNOWN),
            geometry: Geometry::default(),
        }
    }
}

#[derive(Default)]
pub struct AvInfo {
    pub video: Video,
    pub timing: Timing,
}

pub fn try_set_new_geometry(ctx: &Arc<RetroContext>, raw_geometry_ptr: *mut retro_game_geometry) {
    let raw_geometry = unsafe { *raw_geometry_ptr };
    let geometry_ctx = &ctx.core.av_info.video.geometry;

    if raw_geometry.aspect_ratio != *geometry_ctx.aspect_ratio.lock().unwrap()
        || raw_geometry.base_height != *geometry_ctx.base_height.lock().unwrap()
        || raw_geometry.base_width != *geometry_ctx.base_width.lock().unwrap()
    {
        *geometry_ctx.aspect_ratio.lock().unwrap() = raw_geometry.aspect_ratio;
        *geometry_ctx.base_height.lock().unwrap() = raw_geometry.base_height;
        *geometry_ctx.base_width.lock().unwrap() = raw_geometry.base_width;
        *geometry_ctx.max_height.lock().unwrap() = raw_geometry.max_height;
        *geometry_ctx.max_width.lock().unwrap() = raw_geometry.max_width;
    }
}
