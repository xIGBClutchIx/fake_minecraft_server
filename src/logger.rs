use chrono::prelude::{Local};
use ansi_term::Colour::{*};
use log::{Record, Metadata, Level, LevelFilter};
use pad::PadStr;

struct Logger;

impl log::Log for Logger {
    fn enabled(&self, _: &Metadata) -> bool {true}

    fn log(&self, record: &Record) {
        let time_stamp = Cyan.paint(format!("{}", Local::now().format("%H:%M:%S:%3f")));

        let log_color = match record.level() {
            Level::Trace => Purple,
            Level::Debug => Blue,
            Level::Info => Green,
            Level::Warn => Yellow,
            Level::Error => Red
        };

        let unicode_arrow = '\u{e0b0}'.to_string();
        let beginning_arrow = Black.on(log_color).paint(&unicode_arrow);
        let level = Black.on(log_color).paint(format!(" {} ", record.level().as_str().pad_to_width(5)));
        let color_arrow = log_color.paint(&unicode_arrow);

        let message = White.paint(format!("{}", record.args()));

        println!("{} {}{}{} {}", time_stamp, beginning_arrow, level, color_arrow, message);
    }
    fn flush(&self) {}
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
