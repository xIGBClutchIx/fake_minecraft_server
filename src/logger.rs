use chrono::prelude::{Local};
use ansi_term::Colour::{*};
use log::{Record, Metadata, Level, LevelFilter};
use pad::PadStr;

struct Logger;

impl log::Log for Logger {
    fn log(&self, record: &Record) {
        let left_bracket =  Fixed(8).paint("[");
        let time_stamp = Cyan.paint(format!("{}", Local::now().format("%H:%M:%S:%3f")));
        let right_bracket = Fixed(8).paint("]");

        let level = format!("{}", record.level());

        let log_level = match record.level() {
            Level::Trace => Purple.paint(level),
            Level::Debug => Blue.paint(level),
            Level::Info => Green.paint(level.pad_to_width(5)),
            Level::Warn => Yellow.paint(level.pad_to_width(5)),
            Level::Error => Red.paint(level)
        };

        let divider = Fixed(8).paint(">");
        let message = White.paint(format!("{}", record.args()));

        println!("{}{}{} {} {} {}", left_bracket, time_stamp, right_bracket, log_level, divider, message);
    }

    fn flush(&self) {}
    fn enabled(&self, _: &Metadata) -> bool {true}
}

pub fn create_logger() {
    enable_windows_color();
    let _ = log::set_logger(&Logger).map(|()| log::set_max_level(LevelFilter::Trace));
}

#[cfg(windows)]
fn enable_windows_color() {
    let _ = ansi_term::enable_ansi_support();
}

#[cfg(not(windows))]
fn enable_windows_color() {
}
