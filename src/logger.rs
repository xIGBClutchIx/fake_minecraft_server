use ansi_term::Colour::{*};
use chrono::prelude::Local;
use log::{Record, Metadata, Level, LevelFilter};
use pad::PadStr;

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _: &Metadata) -> bool {true}

    fn log(&self, record: &Record) {
        let unicode_arrow = '\u{e0b0}'.to_string();
        let beginning_arrow = Black.on(Blue).paint(&unicode_arrow);

        let time_stamp = Black.on(Blue).paint(format!(" {} ", Local::now().format("%H:%M:%S:%3f")));

        let log_color = match record.level() {
            Level::Trace => Cyan,
            Level::Debug => Purple,
            Level::Info => Green,
            Level::Warn => Yellow,
            Level::Error => Red
        };

        let level = Black.on(log_color).paint(format!(" {} ", record.level().as_str().pad_to_width(5)));
        let separator_arrow = Blue.on(log_color).paint(&unicode_arrow);
        let end_level_arrow = log_color.paint(&unicode_arrow);

        // Hack hide tokio trace
        if !record.args().to_string().contains("registering event source") {
            println!("{}{}{}{}{} {}", beginning_arrow, time_stamp, separator_arrow, level, end_level_arrow, record.args());
        }
    }

    fn flush(&self) {}
}

pub fn create_logger(level: LevelFilter) {
    enable_windows_color();
    let _ = log::set_logger(&Logger).map(|()| log::set_max_level(level));
}

#[cfg(windows)]
fn enable_windows_color() {
    let _ = ansi_term::enable_ansi_support();
}

#[cfg(not(windows))]
fn enable_windows_color() {
}
