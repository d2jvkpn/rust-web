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

pub enum LogOutput<'a> {
    File(&'a str),
    Console,
}

pub fn init_logger(output: LogOutput, level: log::LevelFilter) -> anyhow::Result<()> {
    // if let Err(e) = fs::remove_dir_all("logs") {
    //     if e.kind() != io::ErrorKind::NotFound {
    //         return Err(Error::new(e).context("remove logs/"));
    //    }
    // };

    // fs::create_dir_all("logs").map_err(|e| Error::new(e).context("create logs/"))?;

    let name = match output {
        LogOutput::File(_) => "logfile",
        LogOutput::Console => "stderr",
    };

    // Log Trace level output to file where trace is the default level
    // and the programmatically specified level to stderr.
    // LevelFilter::Trace
    let appender = match output {
        LogOutput::File(v) => {
            // Pattern: https://docs.rs/log4rs/*/log4rs/encode/pattern/index.html
            let logfile = FileAppender::builder()
                .encoder(Box::new(JsonEncoder::new()))
                .build(v)
                .map_err(|e| Error::new(e).context("log4rs create logfile from FileAppender"))?;

            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build(name, Box::new(logfile))
        }
        LogOutput::Console => {
            let stderr = ConsoleAppender::builder().target(Target::Stderr).build();
            Appender::builder()
                .filter(Box::new(ThresholdFilter::new(level)))
                .build(name, Box::new(stderr))
        }
    };

    let config = Config::builder()
        .appender(appender)
        .build(Root::builder().appender(name).build(level))
        .map_err(|e| Error::new(e).context("log4rs config builder"))?;

    // Use this to change log levels at runtime.
    // This means you can change the default log level to trace
    // if you are trying to debug an issue and need more logs on then turn it off
    // once you are done.
    let _handle = log4rs::init_config(config)?; // SetLoggerError

    Ok(())
}
