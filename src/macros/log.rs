/// Logs a debug if log feature is enabled
macro_rules! debug {
    ($($x:tt)*) => (
        #[cfg(feature = "log")] {
            log::debug!($($x)*)
        }
    )
}

pub(crate) use debug;
