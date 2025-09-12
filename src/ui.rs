use crate::render::RenderContext;
use std::fmt::Debug;

use input_lib::Controller;
use macroquad::{
    color::{BLACK, BLUE, GREEN},
    shapes::draw_rectangle,
    text::draw_text,
};

#[derive(Debug)]
pub struct Menu<T: MenuItem> {
    items: Vec<T>,
    selected: usize,
}

impl<T: MenuItem> Menu<T> {
    pub fn new(values: &[T]) -> Self {
        Self {
            items: values.to_vec(),
            selected: 0,
        }
    }
    pub fn update(&mut self, controller: &Controller) {
        let dy = controller.timed_hold().dpad_y;
        self.selected = self
            .selected
            .saturating_add_signed((-dy).try_into().unwrap());
        self.selected = self.selected.clamp(0, self.items.len() - 1);
    }

    pub fn selected(&self) -> &T {
        &self.items[self.selected]
    }

    #[allow(clippy::cast_precision_loss)]
    // TODO Shift menu position based on cursor pos
    pub fn render(&self) {
        let view_rect = RenderContext::screen_view_rect();
        let w = view_rect.w * 0.2;
        let h = view_rect.h / 15.0;
        let x = view_rect.x + (view_rect.w * 0.9) - w;
        let y = view_rect.y + view_rect.h * 0.2;

        self.items.iter().enumerate().for_each(|(i, item)| {
            draw_rectangle(x, y + (i as f32 * h), w, h, BLUE);
            if self.selected == i {
                draw_rectangle(x, y + (i as f32 * h), w, h, GREEN);
            }

            draw_text(item.menu_label(), x, y + ((i + 1) as f32 * h), h, BLACK);
        });
    }
}

pub trait MenuItem: Copy + PartialEq + Eq + Debug {
    fn menu_label(&self) -> &str;
}
