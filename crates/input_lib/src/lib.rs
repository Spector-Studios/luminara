#[warn(clippy::all, clippy::pedantic)]
mod virtual_controller;
pub use self::virtual_controller::ButtonState;
pub use self::virtual_controller::Buttons;
pub use self::virtual_controller::Controller;

pub use enumset;
