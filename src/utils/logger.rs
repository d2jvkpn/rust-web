use anyhow::Error;
// use log::LevelFilter; // debug, error, info, trace, warn
use log4rs::{
    append::console::{ConsoleAppender, Target},
    append::file::FileAppender,
    config::{Appender, Config, Root},
    encode::json::JsonEncoder,
    // encode::pattern::PatternEncoder,
    filter::threshold::ThresholdFilter,
};

pub fn init_logger(file_path: &str, level: log::LevelFilter, console: bool) -> anyhow::Result<()> {
    // if let Err(e) = fs::remove_dir_all("logs") {
    //     if e.kind() != io::ErrorKind::NotFound {
    //         return Err(Error::new(e).context("remove logs/"));
    //    }
    // };

    // fs::create_dir_all("logs").map_err(|e| Error::new(e).context("create logs/"))?;

    // Build a stderr logger.
    let stderr = ConsoleAppender::builder().target(Target::Stderr).build();

    // Logging to log file.
    let logfile = FileAppender::builder()
        // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
        .encoder(Box::new(JsonEncoder::new()))
        .build(file_path)
        .map_err(|e| Error::new(e).context("log4rs create logfile from FileAppender"))?;

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    // LevelFilter::Trace
    let config = if console {
        Config::builder()
            .appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(level)))
                    .build("logfile", Box::new(logfile)),
            )
            .appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(level)))
                    .build("stderr", Box::new(stderr)),
            )
            .build(Root::builder().appender("logfile").appender("stderr").build(level))
            .map_err(|e| Error::new(e).context("log4rs config builder"))?
    } else {
        Config::builder()
            .appender(
                Appender::builder()
                    .filter(Box::new(ThresholdFilter::new(level)))
                    .build("logfile", Box::new(logfile)),
            )
            .build(Root::builder().appender("logfile").build(level))
            .map_err(|e| Error::new(e).context("log4rs config builder"))?
    };

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let _handle = log4rs::init_config(config)?; // SetLoggerError

    Ok(())
}
