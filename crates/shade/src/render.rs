use macroquad::{
    camera::Camera2D,
    math::{UVec2, Vec2, vec2},
    window::{screen_height, screen_width},
};

use crate::logging::internal_i;

pub(crate) struct RenderManager {
    screen_size: Vec2,
    canvas_size: Vec2,
    pub(crate) camera: Camera2D,
}

impl RenderManager {
    const PADDING_FACTOR: f32 = 0.05;
    const FLOAT_TOLERANCE: f32 = 1e-4;
    const Y_OFFSET_FACTOR: f32 = -0.2;

    #[expect(clippy::cast_precision_loss)]
    pub(crate) fn new(canvas_size: UVec2) -> Self {
        Self {
            screen_size: Vec2::ZERO,
            camera: Camera2D {
                offset: vec2(-1.0, 1.0),
                zoom: vec2(2.0 / canvas_size.x as f32, 2.0 / canvas_size.y as f32),
                ..Default::default()
            },
            canvas_size: Vec2::new(canvas_size.x as f32, canvas_size.y as f32),
        }
    }

    pub(crate) fn update(&mut self) {
        let curr_size: Vec2 = (screen_width(), screen_height()).into();
        if (self.screen_size - curr_size).length_squared() > Self::FLOAT_TOLERANCE {
            self.resize(curr_size);
        }
    }

    #[expect(clippy::cast_possible_truncation)]
    fn resize(&mut self, new_size: Vec2) {
        internal_i!("Canvas render resize");

        let (sw, sh) = new_size.into();
        let scale_factor =
            (1.0 - Self::PADDING_FACTOR) * (sw / self.canvas_size.x).min(sh / self.canvas_size.y);

        let rect_size = self.canvas_size * scale_factor;
        let (rect_w, rect_h) = (rect_size.x as i32, rect_size.y as i32);

        let rect_x = (sw as i32 - rect_w) / 2;

        let desired_y = (sh as i32 - rect_h) / 2 - (sh * Self::Y_OFFSET_FACTOR) as i32;
        let rect_y = desired_y.clamp(0, sh as i32 - rect_h);

        self.camera.viewport = Some((rect_x, rect_y, rect_w, rect_h));
    }
}
