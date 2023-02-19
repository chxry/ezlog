use log::{Log, Record, Level, Metadata, SetLoggerError, LevelFilter};
use crossterm::style::Stylize;
use time::OffsetDateTime;

static mut LOGGER: EzLog = EzLog { records: vec![] };

pub struct LogRecord {
  pub time: OffsetDateTime,
  pub module: String,
  pub level: Level,
  pub msg: String,
}

struct EzLog {
  records: Vec<LogRecord>,
}

impl Log for EzLog {
  fn enabled(&self, _: &Metadata) -> bool {
    true
  }

  fn log(&self, record: &Record) {
    let record = LogRecord {
      time: OffsetDateTime::now_utc(),
      module: record.module_path().unwrap_or_default().to_string(),
      level: record.level(),
      msg: format!("{}", record.args()),
    };
    let (h, m, s) = record.time.to_hms();
    let time_mod = format!("{:02}:{:02}:{:02} {}", h, m, s, record.module).dark_grey();
    let level = match record.level {
      Level::Error => "error:".red(),
      Level::Warn => "warn:".yellow(),
      Level::Info => "info:".green(),
      Level::Debug => "debug:".blue(),
      Level::Trace => "trace:".grey(),
    }
    .bold();
    println!("{} {} {}", time_mod, level, record.msg);
    unsafe {
      LOGGER.records.push(record);
    }
  }

  fn flush(&self) {}
}

pub fn init(level: LevelFilter) -> Result<(), SetLoggerError> {
  log::set_max_level(level);
  log::set_logger(unsafe { &LOGGER })
}

pub fn records() -> &'static Vec<LogRecord> {
  unsafe { &LOGGER.records }
}
