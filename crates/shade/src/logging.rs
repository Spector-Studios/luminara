macro_rules! internal_w {
    ($($arg:tt)*) => {
        #[cfg(feature="debug-log")] {
            macroquad::logging::warn!(
                "[ShadeEngine: {}:{}] {}",
                file!(),
                line!(),
                format_args!($($arg)*)
            );
        }
    };
}

macro_rules! internal_i {
    ($($arg:tt)*) => {
        #[cfg(feature = "debug-log")] {
            macroquad::logging::info!(
                "[ShadeEngine: {}:{}] {}",
                file!(),
                line!(),
                format_args!($($arg)*)
            );
        }
    };
}

macro_rules! internal_e {
    ($($arg:tt)*) => {
        #[cfg(feature="debug-log")] {
            macroquad::logging::error!(
                "[ShadeEngine: {}:{}] {}",
                file!(),
                line!(),
                format_args!($($arg)*)
            );
        }
    };
}

macro_rules! internal_d {
    ($($arg:tt)*) => {
        #[cfg(feature="debug-log")] {
            macroquad::logging::debug!(
                "[ShadeEngine: {}:{}] {}",
                file!(),
                line!(),
                format_args!($($arg)*)
            );
        }
    };
}

pub(crate) use internal_d;
pub(crate) use internal_e;
pub(crate) use internal_i;
pub(crate) use internal_w;
