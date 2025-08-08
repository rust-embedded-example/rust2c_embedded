use core::ffi::{c_char, c_int, c_uint};

use alloc::format;
use alloc::vec::Vec;
use log::{Level, LevelFilter, Metadata, Record, SetLoggerError};

struct SimpleLogger;

impl log::Log for SimpleLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        metadata.level() <= Level::Debug
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            // 映射到 C 侧 level（保持当前数值映射）
            let log_level = match record.level() {
                Level::Error => 3u32,
                Level::Warn => 2u32,
                Level::Info => 1u32,
                Level::Debug => 0u32,
                Level::Trace => 1u32,
            };

            // 构建带换行且以 NUL 结尾的消息缓冲
            let log_str = format!("{}", record.args());
            let mut buf: Vec<u8> = Vec::with_capacity(log_str.len() + 2);
            buf.extend_from_slice(log_str.as_bytes());
            buf.push(b'\n');
            buf.push(0);

            // 文件名与行号
            let file_name = record.file().unwrap_or("unknown");
            let line_number = record.line().unwrap_or(0);
            let file_name_cstr = format!("{}\0", file_name);

            unsafe {
                sys_log(
                    1, // AllLogType
                    log_level,
                    file_name_cstr.as_ptr() as *const c_char,
                    line_number as c_int,
                    b"%s\0".as_ptr() as *const c_char,
                    buf.as_ptr() as *const c_char,
                );
            }
        }
    }

    fn flush(&self) {}
}

static LOGGER: SimpleLogger = SimpleLogger;

pub fn init() -> Result<(), SetLoggerError> {
    log::set_logger(&LOGGER).map(|()| log::set_max_level(LevelFilter::Debug))
}

unsafe extern "C" {
    fn sys_log(
        log_type: c_uint,
        level: c_uint,
        file: *const c_char,
        line: c_int,
        format: *const c_char,
        ...,
    ) -> c_int;
}

