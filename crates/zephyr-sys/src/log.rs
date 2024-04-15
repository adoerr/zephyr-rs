use alloc::format;

use log::{LevelFilter, Metadata, Record};

use crate::k_str_out;

pub fn init(max_level: LevelFilter) {
    log::set_logger(&LOGGER).unwrap();
    log::set_max_level(max_level);
}

static LOGGER: Logger = Logger;
struct Logger;

impl log::Log for Logger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let msg = format!("{} {}: {}", record.level(), record.target(), record.args());
            unsafe {
                k_str_out(msg.as_ptr(), msg.len());
            }
        }
    }

    fn flush(&self) {}
}
