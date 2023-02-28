mod db;
mod handlers;
mod internal;
mod middlewares;
mod models;
mod utils;

use internal::load_config;
use sqlx::PgPool;
use std::io;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "rust-web", about = "a rust web app")]
struct Opts {
    #[structopt(long, default_value = "configs/local.yaml", help = "configuration file path")]
    config: String,

    #[structopt(long = "address", default_value = "127.0.0.1", help = "http server address")]
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

    config.configuration = opts.config;
    config.release = opts.release;

    config.threads = opts.threads;
    let (cpus, _) = utils::number_of_cpus();
    if config.threads == 0 || config.threads > cpus {
        config.threads = cpus;
    }
    dbg!(&config);

    let pool = PgPool::connect(&config.database.to_string())
        .await
        .expect("Failed to connect to Postgres.");

    internal::startup::run(&address, pool)?.await
}
