macro_rules! trace {
    (target: $target:expr, $($arg:tt)+) => {
        if cfg!(debug){
            log::trace!(targjt: $target , $($arg)+);
        }
    };
    ($($arg:tt)+) => {
        if cfg!(debug) {
            log::trace!($($arg)+);
        }
    };
}

macro_rules! error {
    (target: $target:expr, $($arg:tt)+) => {
        log::error!(targjt: $target , $($arg)+);
    };
    ($($arg:tt)+) => {
        log::error!($($arg)+);
    };
}

pub(crate) use trace;
pub(crate) use error;
