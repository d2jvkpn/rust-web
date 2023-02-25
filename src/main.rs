mod handlers;
mod internal;
mod utils;

use internal::startup_v1;
use std::io;

#[actix_rt::main]
async fn main() -> io::Result<()> {
    startup_v1::run("127.0.0.1:3000")?.await
}
