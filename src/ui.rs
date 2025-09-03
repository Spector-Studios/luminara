use input_lib::{ButtonState, Controller};
use macroquad::{
    color::{BLACK, BLUE, GREEN},
    shapes::draw_rectangle,
    text::draw_text,
};

use crate::render::RenderContext;

// pub struct UiState {
//     pub menu: Option<Menu>,
// }

// impl UiState {
//     pub fn empty() -> Self {
//         Self { menu: None }
//     }
//     pub fn new() -> Self {
//         Self {
//             menu: Some(Menu {
//                 items: vec![MenuItem::wait(), MenuItem::attack()],
//                 selected: 0,
//             }),
//         }
//     }
//     pub fn update(&mut self, input: &ButtonState) {
//         if let Some(menu) = &mut self.menu {
//             menu.update(input);
//         }
//     }

//     pub fn render(&self, render_ctx: &RenderContext) {
//         if let Some(menu) = &self.menu {
//             menu.render(render_ctx);
//         }
//     }
// }

#[derive(Debug)]
pub struct Menu {
    items: Vec<MenuItem>,
    selected: usize,
}

impl Menu {
    pub fn new(values: &[&str]) -> Self {
        Self {
            items: values.iter().map(|value| MenuItem::new(value)).collect(),
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

    pub fn selected(&self) -> &str {
        &self.items[self.selected].label
    }

    #[allow(clippy::cast_precision_loss)]
    // TODO Shift menu position based on cursor pos
    pub fn render(&self, render_ctx: &RenderContext) {
        let view_rect = render_ctx.view_rect();
        let w = view_rect.w * 0.2;
        let h = view_rect.h / 15.0;
        let x = view_rect.x + (view_rect.w * 0.9) - w;
        let y = view_rect.y + view_rect.h * 0.2;

        self.items.iter().enumerate().for_each(|(i, item)| {
            draw_rectangle(x, y + (i as f32 * h), w, h, BLUE);
            if self.selected == i {
                draw_rectangle(x, y + (i as f32 * h), w, h, GREEN);
            }

            draw_text(&item.label, x, y + ((i + 1) as f32 * h), h, BLACK);
        });
    }
}

#[derive(PartialEq, Eq, Debug)]
pub struct MenuItem {
    label: String,
}

impl MenuItem {
    pub fn new(label: &str) -> Self {
        Self {
            label: label.to_string(),
        }
    }
}
