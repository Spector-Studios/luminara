use enumset::{EnumSet, EnumSetType};

pub struct InputManager {
    curr_input: Input,
    last_input: Input,
}
impl InputManager {
    #[inline]
    pub(crate) fn new() -> Self {
        Self {
            curr_input: Input::new(),
            last_input: Input::new(),
        }
    }

    #[inline]
    #[must_use]
    pub fn get_curr_input(&self) -> Input {
        self.curr_input
    }

    #[inline]
    #[must_use]
    pub fn get_last_input(&self) -> Input {
        self.last_input
    }

    #[inline]
    #[must_use]
    pub fn is_button_pressed(&self, btn: Button) -> bool {
        self.curr_input.buttons.contains(btn)
    }

    #[inline]
    #[must_use]
    pub fn is_button_clicked(&self, btn: Button) -> bool {
        self.curr_input
            .buttons
            .difference(self.last_input.buttons)
            .contains(btn)
    }

    pub(crate) fn update(&mut self) {
        todo!()
    }

    pub(crate) fn render(&mut self) {
        todo!()
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Input {
    pub dpad_x: i8,
    pub dpad_y: i8,
    pub buttons: EnumSet<Button>,
}

impl Input {
    fn new() -> Self {
        Self {
            dpad_x: 0,
            dpad_y: 0,
            buttons: EnumSet::empty(),
        }
    }
}

#[derive(EnumSetType, Debug)]
pub enum Button {
    A,
    B,
    X,
    Y,
    L,
    R,
    Start,
    Select,
}
