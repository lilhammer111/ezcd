use std::error::Error;
use std::fs::File;
use std::io::Write;
use crate::cst::LOG_PATH;
use chrono::Local;
use crate::util::load_file;
#[macro_export]
macro_rules! debug_eprintln {
    ($($arg:tt)*) => {
        #[cfg(feature = "logging")]
        eprintln!($($arg)*);
    };
}

pub fn write(err: Box<dyn Error>) {
    let err_info = err.to_string();

    let datetime = Local::now()
        .format("%Y-%m-%d %H:%M:%S")
        .to_string();

    // let duration = SystemTime::now()
    //     .duration_since(SystemTime::UNIX_EPOCH)
    //     .expect("Time went backwards");
    // let total_seconds = duration.as_secs();
    // let hours = total_seconds / 3600 % 24;
    // let minutes = total_seconds / 60 % 60;
    // let seconds = total_seconds % 60;
    // let datetime = format!(
    //     "{:02}:{:02}:{:02}",
    //     hours,
    //     minutes,
    //     seconds
    // );

    let log = format!(
        "ERROR\t{}\t{}\n",
        datetime,
        err_info
    );

    let log_file = load_file(LOG_PATH);

    let mut f = File::options().append(true).create(true).open(log_file)
        .expect("Failed to open log file");

    f.write_all(log.as_bytes()).expect("Failed to write log");
}