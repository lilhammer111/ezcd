#[macro_export]
macro_rules! debug_eprintln {
    ($($arg:tt)*) => {
        #[cfg(feature = "logging")]
        eprintln!($($arg)*);
    };
}
