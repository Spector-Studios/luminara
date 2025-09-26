#[cfg(feature = "debug-log")]
macro_rules! internal_w {
    (target: $target:expr, $($arg:tt)+) => (
        macroquad::logging::warn!(target: $target,$($arg)+);
    );
    ($($arg:tt)+) => (
        macroquad::logging::warn!("[Shade Engine; {}:{}] {}",file!(),line!(),$($arg)+);
    )
}

#[cfg(not(feature = "debug-log"))]
macro_rules! internal_w {
    (target: $target:expr, $($arg:tt)+) => {};
    ($($arg:tt)+) => {};
}

macro_rules! internal_i {
    (target: $target:expr, $($arg:tt)+) => (
        macroquad::logging::info!(target: $target,$($arg)+);
    );
    ($($arg:tt)+) => (
        macroquad::logging::info!("[Shade Engine; {}:{}] {}",file!(),line!(),$($arg)+);
    )
}

#[cfg(not(feature = "debug-log"))]
macro_rules! internal_i {
    (target: $target:expr, $($arg:tt)+) => {};
    ($($arg:tt)+) => {};
}

macro_rules! internal_e {
    (target: $target:expr, $($arg:tt)+) => (
        macroquad::logging::error!(target: $target,$($arg)+);
    );
    ($($arg:tt)+) => (
        macroquad::logging::error!("[Shade Engine; {}:{}] {}",file!(),line!(),$($arg)+);
    )
}

#[cfg(not(feature = "debug-log"))]
macro_rules! internal_e {
    (target: $target:expr, $($arg:tt)+) => {};
    ($($arg:tt)+) => {};
}

macro_rules! internal_d {
    (target: $target:expr, $($arg:tt)+) => (
        macroquad::logging::debug!(target: $target,$($arg)+);
    );
    ($($arg:tt)+) => (
        macroquad::logging::debug!("[Shade Engine; {}:{}] {}",file!(),line!(),$($arg)+);
    )
}

#[cfg(not(feature = "debug-log"))]
macro_rules! internal_d {
    (target: $target:expr, $($arg:tt)+) => {};
    ($($arg:tt)+) => {};
}

pub(crate) use internal_d;
pub(crate) use internal_e;
pub(crate) use internal_i;
pub(crate) use internal_w;
