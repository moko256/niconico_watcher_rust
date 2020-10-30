use log::*;
use std::io::{self, Write};

static APP_LOGGER: AppLogger = AppLogger;
pub struct AppLogger;
impl AppLogger {
    fn to_severity_rfc5424(level: Level) -> usize {
        match level {
            Level::Trace => 7,
            Level::Debug => 7,
            Level::Info => 6,
            Level::Warn => 4,
            Level::Error => 3,
        }
    }

    pub fn init() -> Result<(), SetLoggerError> {
        set_max_level(LevelFilter::Info);
        set_logger(&APP_LOGGER)?;
        Ok(())
    }
}
impl Log for AppLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        (metadata.target() == "nicow" && metadata.level() >= Level::Info)
            || metadata.level() >= Level::Warn
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            println!(
                "<{}>{} - {}",
                AppLogger::to_severity_rfc5424(record.level()),
                record.target(),
                record.args()
            );
        }
    }

    fn flush(&self) {}
}
