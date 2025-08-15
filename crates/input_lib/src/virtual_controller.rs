use enumset::EnumSet;
use enumset::EnumSetType;
use macroquad::prelude::*;
use macroquad_ex_ui::XButton;

#[derive(EnumSetType, Debug)]
pub enum Buttons {
    A,
    B,
    X,
    Y,
    Start,
    Select,
}

#[derive(Clone, Copy, Debug)]
pub enum DPadButtons {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Clone, Copy, Debug)]
pub enum ButtonKind {
    DPad(DPadButtons),
    Action(Buttons),
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ButtonState {
    pub dpad_x: i32,
    pub dpad_y: i32,
    pub buttons: EnumSet<Buttons>,
}

impl ButtonState {
    #[must_use]
    pub fn new() -> Self {
        Self {
            dpad_x: 0,
            dpad_y: 0,
            buttons: EnumSet::empty(),
        }
    }

    pub fn reset(&mut self) {
        self.dpad_x = 0;
        self.dpad_y = 0;
        self.buttons.clear();
    }

    fn set(&mut self, button: ButtonKind) {
        match button {
            ButtonKind::DPad(dpad_buttons) => match dpad_buttons {
                DPadButtons::Left => self.dpad_x -= 1,
                DPadButtons::Right => self.dpad_x += 1,
                DPadButtons::Up => self.dpad_y += 1,
                DPadButtons::Down => self.dpad_y -= 1,
            },
            ButtonKind::Action(buttons) => {
                self.buttons.insert(buttons);
            }
        }
    }
}

impl Default for ButtonState {
    fn default() -> Self {
        Self::new()
    }
}

pub struct Controller {
    buttons: [(XButton, ButtonKind); 10],
    pub button_state: ButtonState,
}

impl Controller {
    #[must_use]
    pub fn new() -> Self {
        let sw = screen_width();
        let sh = screen_height();

        Self {
            buttons: [
                (
                    xbutton(200.0, sh - 400.0, 100.0, 100.0, "↑"),
                    ButtonKind::DPad(DPadButtons::Up),
                ),
                (
                    xbutton(200.0, sh - 200.0, 100.0, 100.0, "↓"),
                    ButtonKind::DPad(DPadButtons::Down),
                ),
                (
                    xbutton(100.0, sh - 300.0, 100.0, 100.0, "←"),
                    ButtonKind::DPad(DPadButtons::Left),
                ),
                (
                    xbutton(300.0, sh - 300.0, 100.0, 100.0, "→"),
                    ButtonKind::DPad(DPadButtons::Right),
                ),
                (
                    xbutton(sw - 150.0 + 10.0, sh - 350.0 - 10.0, 100.0, 100.0, "A"),
                    ButtonKind::Action(Buttons::A),
                ),
                (
                    xbutton(sw - 250.0 - 10.0, sh - 250.0 + 10.0, 100.0, 100.0, "B"),
                    ButtonKind::Action(Buttons::B),
                ),
                (
                    xbutton(sw - 250.0 - 10.0, sh - 350.0 - 10.0, 100.0, 100.0, "X"),
                    ButtonKind::Action(Buttons::X),
                ),
                (
                    xbutton(sw - 150.0 + 10.0, sh - 250.0 + 10.0, 100.0, 100.0, "Y"),
                    ButtonKind::Action(Buttons::Y),
                ),
                (
                    xbutton(sw / 2.0 - 50.0 - 100.0, sh - 600.0, 100.0, 50.0, "Start"),
                    ButtonKind::Action(Buttons::Start),
                ),
                (
                    xbutton(sw / 2.0 - 50.0 + 100.0, sh - 600.0, 100.0, 50.0, "Select"),
                    ButtonKind::Action(Buttons::Select),
                ),
            ],
            button_state: ButtonState::new(),
        }
    }

    pub fn update(&mut self) {
        self.button_state.reset();
        for (btn, flag) in &mut self.buttons {
            btn.update();
            if btn.is_pressed {
                self.button_state.set(*flag);
            }
        }
    }

    pub fn draw(&self, font: Option<&Font>) {
        for (btn, _) in &self.buttons {
            btn.draw(font);
        }
    }
}

impl Default for Controller {
    fn default() -> Self {
        Self::new()
    }
}

#[inline]
fn xbutton(x: f32, y: f32, w: f32, h: f32, label: &str) -> XButton {
    XButton::new(Rect::new(x, y, w, h), label, RED)
}
