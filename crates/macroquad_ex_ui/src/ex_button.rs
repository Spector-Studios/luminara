//use macroquad::{color::Color, input::touches, math::Rect, shapes::draw_rectangle};
use macroquad::prelude::*;

#[derive(Debug, Clone)]
pub struct XButton {
    rect: Rect,
    label: String,
    color: Color,
    pub is_pressed: bool,
}

impl XButton {
    pub fn new(rect: Rect, label: &str, color: Color) -> Self {
        Self {
            rect,
            label: label.to_string(),
            color,
            is_pressed: false,
        }
    }

    pub fn update(&mut self) {
        self.is_pressed = touches()
            .iter()
            .any(|touch| self.rect.contains(touch.position));
    }

    pub fn draw(&self, font: Option<&Font>) {
        draw_rectangle(
            self.rect.x,
            self.rect.y,
            self.rect.w,
            self.rect.h,
            self.color,
        );

        let text_center = get_text_center(&self.label, None, 50, 1.0, 0.0);
        draw_text_ex(
            &self.label,
            self.rect.center().x - text_center.x,
            self.rect.center().y - text_center.y,
            TextParams {
                font,
                font_size: 50,
                color: BLACK,
                ..Default::default()
            },
        );
    }
}
