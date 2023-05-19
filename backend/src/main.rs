mod db;
mod handlers;
mod internal;
mod middlewares;
mod models;
mod utils;

use internal::{load_config, settings, Configuration};
use log::LevelFilter::{Debug, Info};
use sqlx::{postgres::PgConnectOptions, ConnectOptions, PgPool};
use std::{io, path::Path, str::FromStr};
use structopt::StructOpt;
use utils::{init_logger, LogOutput};

#[derive(Debug, StructOpt)]
#[structopt(name = "rust-backend", about = "a rust web backend app")]
pub struct Opts {
    #[structopt(long, default_value = "configs/local.yaml", help = "configuration file path")]
    config: String,

    #[structopt(long = "addr", default_value = "0.0.0.0", help = "http server ip address")]
    addr: String,

    #[structopt(long, default_value = "3011", help = "http server port")]
    port: u16,

    #[structopt(long, default_value = "0", help = "threads limit")]
    threads: usize,

    #[structopt(long, help = "run in release mode")]
    release: bool,
}

#[actix_rt::main]
async fn main() -> io::Result<()> {
    let opts = Opts::from_args();
    let mut config = load_config(&opts.config).unwrap();
    read_opts(&mut config, opts);

    let dsn = config.database.to_string();
    let address = config.address.clone();

    let name = Path::new(&config.file_path).with_extension("");
    // let name = name.file_name().map(|v| v.to_str().or(Some(""))).unwrap_or(Some("")).unwrap_or("");
    let name = match name.file_name() {
        None => "",
        Some(v) => v.to_str().unwrap_or(""),
    };

    let pool = if config.release {
        let log_file = format!("logs/{}.{}.log", env!("CARGO_PKG_NAME"), name);
        init_logger(LogOutput::File(log_file.as_ref()), Info).unwrap();

        println!("=== Http Server is listening on {address:?}");
        let options = PgConnectOptions::from_str(&dsn).unwrap().disable_statement_logging().clone();
        PgPool::connect_with(options).await.unwrap()
    } else {
        init_logger(LogOutput::Console, Debug).unwrap();

        dbg!(&config);
        PgPool::connect(&dsn).await.expect("Failed to connect to Postgres.")
    };

    sqlx::migrate!("./migrations").run(&pool).await.expect("Failed to migrate the database");

    utils::GitBuildInfo::set(include_str!("build-info.yaml")).unwrap();

    settings::Settings::set(config, pool.clone()).unwrap();

    internal::startup::run(&address, pool)?.await
}

fn read_opts(config: &mut Configuration, opts: Opts) {
    let (cpus, _) = utils::number_of_cpus();

    config.address = format!("{}:{}", opts.addr, opts.port);
    config.file_path = opts.config;
    config.release = opts.release;
    config.threads = if config.threads == 0 || config.threads > cpus { cpus } else { opts.threads };
}
