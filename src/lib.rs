#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

use log::{Level, Metadata, Record};

struct AsyncLogger;

impl log::Log for AsyncLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Info
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            //println!("{} - {}", record.level(), record.args());
            println!("{}", color_format(record));
        }
    }

    fn flush(&self) {}
}

fn color_format(record: &Record) -> String {
    let level = record.level();
    format!(
        "[{}] {} [{}] {}:{}: {}",
        //style(level, now.now().format("%Y-%m-%d %H:%M:%S%.6f %:z")),
        style(level, chrono::Local::now()),
        style(level, record.level()),
        record.module_path().unwrap_or("<unnamed>"),
        record.file().unwrap_or("<unnamed>"),
        record.line().unwrap_or(0),
        style(level, &record.args())
    )
}

/// Helper function that is used in the provided colored format functions.
///
/// Only available with feature `colors`.
fn style<T>(level: log::Level, item: T) -> yansi::Paint<T> {
    match level {
        log::Level::Error => yansi::Paint::fixed(196, item).bold(),
        log::Level::Warn => yansi::Paint::fixed(208, item).bold(),
        log::Level::Info => yansi::Paint::new(item),
        log::Level::Debug => yansi::Paint::fixed(7, item),
        log::Level::Trace => yansi::Paint::fixed(8, item),
    }
}

use log::{LevelFilter, SetLoggerError};

static LOGGER: AsyncLogger = AsyncLogger;

pub fn init() -> Result<(), SetLoggerError> {
    // disable coloring on on support platform
    if cfg!(windows) && !yansi::Paint::enable_windows_ascii() {
        yansi::Paint::disable();
    }

    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Info))
}
