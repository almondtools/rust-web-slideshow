use env_logger::{TimestampPrecision, WriteStyle};
use log::LevelFilter;

pub fn init() {
    env_logger::builder()
        .filter_level(LevelFilter::Debug)
        .write_style(WriteStyle::Always)
        .format_timestamp(Some(TimestampPrecision::Micros))
        .init();
}
