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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ButtonState {
    pub dpad_x: i32,
    pub dpad_y: i32,
    pub buttons: EnumSet<Buttons>,
}

impl ButtonState {
    #[must_use]
    fn new() -> Self {
        Self {
            dpad_x: 0,
            dpad_y: 0,
            buttons: EnumSet::empty(),
        }
    }

    fn reset(&mut self) {
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

const INITIAL_DELAY: f32 = 0.25;
const REPEAT_DELAY: f32 = 0.06;
pub struct Controller {
    // TODO This should probably be a Vec or HashMap
    buttons: [(XButton, ButtonKind); 10],
    button_state: ButtonState,
    last_state: ButtonState,
    screen_width: f32,
    screen_height: f32,
    timer: f32,
}

impl Controller {
    #[must_use]
    pub fn new() -> Self {
        let mut controller = Self {
            screen_width: 0.0,
            screen_height: 0.0,
            timer: 0.0,
            button_state: ButtonState::new(),
            last_state: ButtonState::new(),
            buttons: [
                (xbutton("↑"), ButtonKind::DPad(DPadButtons::Up)),
                (xbutton("↓"), ButtonKind::DPad(DPadButtons::Down)),
                (xbutton("←"), ButtonKind::DPad(DPadButtons::Left)),
                (xbutton("→"), ButtonKind::DPad(DPadButtons::Right)),
                (xbutton("A"), ButtonKind::Action(Buttons::A)),
                (xbutton("B"), ButtonKind::Action(Buttons::B)),
                (xbutton("X"), ButtonKind::Action(Buttons::X)),
                (xbutton("Y"), ButtonKind::Action(Buttons::Y)),
                (xbutton("Start"), ButtonKind::Action(Buttons::Start)),
                (xbutton("Select"), ButtonKind::Action(Buttons::Select)),
            ],
        };

        controller.resize();
        controller
    }

    fn resize(&mut self) {
        info!("Controller resize requested.");

        let sw = screen_width();
        let sh = screen_height();
        self.screen_width = sw;
        self.screen_height = sh;

        let btn_size = f32::max(sh, sw) * 0.05;
        let bar_btn_size = (btn_size * 1.5, btn_size * 0.75);

        let (dpad_x, dpad_y) = (sw * 0.06 + btn_size, sh * 0.85 - btn_size);
        let (act_x, act_y) = (sw * 0.94 - 2.0 * btn_size, sh * 0.85 - btn_size);
        let (bar_x, bar_y) = (sw * 0.5, sh * 0.95 - bar_btn_size.1);

        for (btn, kind) in &mut self.buttons {
            match kind {
                ButtonKind::DPad(DPadButtons::Up) => {
                    btn.rect = Rect::new(dpad_x, dpad_y - btn_size, btn_size, btn_size);
                }
                ButtonKind::DPad(DPadButtons::Down) => {
                    btn.rect = Rect::new(dpad_x, dpad_y + btn_size, btn_size, btn_size);
                }
                ButtonKind::DPad(DPadButtons::Left) => {
                    btn.rect = Rect::new(dpad_x - btn_size, dpad_y, btn_size, btn_size);
                }
                ButtonKind::DPad(DPadButtons::Right) => {
                    btn.rect = Rect::new(dpad_x + btn_size, dpad_y, btn_size, btn_size);
                }

                ButtonKind::Action(Buttons::A) => {
                    btn.rect = Rect::new(act_x + btn_size, act_y, btn_size, btn_size);
                }
                ButtonKind::Action(Buttons::B) => {
                    btn.rect = Rect::new(act_x, act_y + btn_size, btn_size, btn_size);
                }

                ButtonKind::Action(Buttons::X) => {
                    btn.rect = Rect::new(act_x, act_y - btn_size, btn_size, btn_size);
                }

                ButtonKind::Action(Buttons::Y) => {
                    btn.rect = Rect::new(act_x - btn_size, act_y, btn_size, btn_size);
                }

                ButtonKind::Action(Buttons::Start) => {
                    btn.rect = Rect::new(
                        bar_x - bar_btn_size.0 * 1.5,
                        bar_y,
                        bar_btn_size.0,
                        bar_btn_size.1,
                    );
                }

                ButtonKind::Action(Buttons::Select) => {
                    btn.rect = Rect::new(
                        bar_x + bar_btn_size.0 * 0.5,
                        bar_y,
                        bar_btn_size.0,
                        bar_btn_size.1,
                    );
                }
            }
        }
    }

    pub fn update(&mut self) {
        if (self.screen_width - screen_width()).abs() > 1.0
            || (self.screen_height - screen_height()).abs() > 1.0
        {
            self.resize();
        }

        self.last_state = self.button_state;
        self.button_state.reset();
        for (btn, flag) in &mut self.buttons {
            btn.update();
            if btn.is_pressed() {
                self.button_state.set(*flag);
            }
        }

        if self.button_state == ButtonState::default() || self.button_state != self.last_state {
            self.timer = 0.0;
        } else {
            self.timer += get_frame_time();
        }
    }

    #[inline]
    #[must_use]
    pub fn button_state(&self) -> ButtonState {
        self.button_state
    }

    #[inline]
    #[must_use]
    pub fn timed_hold(&self) -> ButtonState {
        if self.button_state != self.last_state
            || (self.timer > INITIAL_DELAY
                && (self.timer - INITIAL_DELAY) % REPEAT_DELAY < get_frame_time())
        {
            return self.button_state;
        }
        ButtonState::default()
    }

    #[inline]
    #[must_use]
    pub fn last_state(&self) -> ButtonState {
        self.last_state
    }

    #[inline]
    #[must_use]
    pub fn clicked(&self, button: Buttons) -> bool {
        self.button_state.buttons.contains(button) && !self.last_state.buttons.contains(button)
    }

    #[inline]
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
fn xbutton(label: &str) -> XButton {
    XButton::new(Rect::new(0.0, 0.0, 0.0, 0.0), label, RED)
}
