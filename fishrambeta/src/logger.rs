use clap_verbosity_flag::LevelFilter;
use slog::{info, o, Drain, Duplicate, Level, Logger};
use slog_async::Async;
use slog_term::{CompactFormat, CountingWriter, FullFormat, PlainDecorator, TermDecorator};
use std::fs::OpenOptions;
use std::io::Write;

#[cfg(not(all(target_arch = "wasm32", target_os = "unknown")))]
pub fn new(f_out: Option<String>, verbosity: Option<clap_verbosity_flag::Verbosity>) -> Logger {
    let term_decorator = TermDecorator::new().build();
    let term_drain = FullFormat::new(term_decorator).build().fuse();
    let llv = match verbosity{
        Some(verbosity) => verbosity.log_level_filter(),
        None => LevelFilter::Debug,
    };
    let log_level = match llv {
        LevelFilter::Off => Level::Critical,
        LevelFilter::Error => Level::Error,
        LevelFilter::Warn => Level::Warning,
        LevelFilter::Info => Level::Info,
        LevelFilter::Debug => Level::Debug,
        LevelFilter::Trace => Level::Trace,
    };
    let async_term_drain =
        slog::LevelFilter::new(Async::new(term_drain).build().fuse(), log_level).fuse();
    let logger = if let Some(file) = f_out {
        let log_file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open(file)
            .unwrap();
        let file_decorator = PlainDecorator::new(log_file);
        let file_drain = CompactFormat::new(file_decorator)
            .use_custom_header_print(|_, mut rd, record, _| {
                unsafe { rd.start_msg().unwrap_unchecked() };
                let mut count_rd = CountingWriter::new(&mut rd);
                unsafe { write!(count_rd, "{}", record.msg()).unwrap_unchecked() };
                Ok(true)
            })
            .build()
            .fuse();
        let async_file_drain =
            slog::LevelFilter::new(Async::new(file_drain).build().fuse(), Level::Info);
        let combined_drain = Duplicate::new(async_term_drain, async_file_drain).fuse();
        Logger::root(combined_drain, o!())
    } else {
        Logger::root(async_term_drain, o!())
    };
    info!(logger, "Using log level, {}", log_level);
    return logger;
}
#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
pub fn new(f_out: Option<String>, verbosity: Option<clap_verbosity_flag::Verbosity>) -> Logger{
    let term_decorator = TermDecorator::new().build();
    let term_drain = FullFormat::new(term_decorator).build().fuse();
    let llv = match verbosity{
        Some(verbosity) => verbosity.log_level_filter(),
        None => LevelFilter::Debug,
    };
    let log_level = match llv {
        LevelFilter::Off => Level::Critical,
        LevelFilter::Error => Level::Error,
        LevelFilter::Warn => Level::Warning,
        LevelFilter::Info => Level::Info,
        LevelFilter::Debug => Level::Debug,
        LevelFilter::Trace => Level::Trace,
    };
    let async_term_drain =
    slog::LevelFilter::new(Async::new(term_drain).build().fuse(), log_level).fuse();
    Logger::root(async_term_drain, o!())
}