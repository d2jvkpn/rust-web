mod db;
mod handlers;
mod internal;
mod middlewares;
mod models;
mod utils;

use internal::{load_config, settings};
use log::LevelFilter;
use sqlx::{postgres::PgConnectOptions, ConnectOptions, PgPool};
use std::{io, str::FromStr};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rust-web", about = "a rust web app")]
struct Opts {
    #[structopt(long, default_value = "configs/local.yaml", help = "configuration file path")]
    config: String,

    #[structopt(long = "addr", default_value = "127.0.0.1", help = "http server ip address")]
    addr: String,

    #[structopt(long, default_value = "3000", help = "http server port")]
    port: u16,

    #[structopt(long, default_value = "0", help = "threads limit")]
    threads: usize,

    #[structopt(long, help = "run in release mode")]
    release: bool,
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let opts = Opts::from_args();
    let address = format!("{}:{}", opts.addr, opts.port);

    let mut config = load_config(&opts.config)
        .unwrap_or_else(|e| panic!("read configuration {}: {:?}", &opts.config, e));

    config.file_path = opts.config;
    config.release = opts.release;

    let log_file = format!("logs/{}.log", env!("CARGO_PKG_NAME"));
    if opts.release {
        utils::init_logger(&log_file, LevelFilter::Info, false).unwrap();
    } else {
        utils::init_logger(&log_file, LevelFilter::Debug, true).unwrap();
    }

    config.threads = opts.threads;
    let (cpus, _) = utils::number_of_cpus();
    if config.threads == 0 || config.threads > cpus {
        config.threads = cpus;
    }
    dbg!(&config);

    let pool = if config.release {
        let options = PgConnectOptions::from_str(&config.database.to_string())
            .unwrap()
            .disable_statement_logging()
            .clone();

        PgPool::connect_with(options).await.unwrap()
    } else {
        PgPool::connect(&config.database.to_string()).await.expect("Failed to connect to Postgres.")
    };

    settings::Settings::set(config, pool.clone()).unwrap();
    internal::startup::run(&address, pool)?.await
}
