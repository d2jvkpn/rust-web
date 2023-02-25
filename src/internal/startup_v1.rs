use crate::handlers::route;
use crate::internal::AppState;
use actix_web::{
    dev::Server,
    middleware::{Compress, NormalizePath},
    web, App, HttpServer,
};
use std::{io, net::TcpListener, time::Duration};

#[allow(dead_code)]
pub fn run(address: &str) -> io::Result<Server> {
    let app_data = web::Data::new(AppState::new());

    println!("=== Http Server is listening on {address:?}");

    let app = move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(Compress::default())
            .wrap(NormalizePath::default())
            .configure(route)
    };

    // HttpServer::new(app)
    //     .keep_alive(Duration::from_secs(60))
    //     .bind(address)?
    //     .run()
    //     .await

    let server = HttpServer::new(app)
        .keep_alive(Duration::from_secs(60))
        .bind(address)?
        .run();

    Ok(server)
}

#[allow(dead_code)]
pub fn run_with_listener(listener: TcpListener) -> io::Result<Server> {
    let app_data = web::Data::new(AppState::new());

    let app = move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(Compress::default())
            .wrap(NormalizePath::default())
            .configure(route)
    };

    let server = HttpServer::new(app)
        .keep_alive(Duration::from_secs(60))
        .listen(listener)?
        .run();

    Ok(server)
}
