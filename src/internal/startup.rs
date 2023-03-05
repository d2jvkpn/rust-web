use super::data::AppState;
use crate::{
    handlers::route,
    middlewares::{no_route_error, Logger, SimpleLogger},
};
use actix_cors::Cors;
use actix_web::{
    dev::Server,
    http::{header, StatusCode},
    middleware::{Compress, ErrorHandlers},
    web, App, HttpServer,
};
use sqlx::PgPool;
use std::{io, net::TcpListener, time::Duration};

pub fn run(address: &str, pool: PgPool) -> io::Result<Server> {
    let app_data = web::Data::new(AppState::new(pool));

    println!("=== Http Server is listening on {address:?}");

    let app = move || {
        // println!("--> new worker in thread: {:?}", std::thread::current().id());
        App::new()
            .app_data(app_data.clone())
            .wrap(ErrorHandlers::new().handler(StatusCode::NOT_FOUND, no_route_error))
            .wrap(Logger {})
            .wrap(
                Cors::default()
                    // add specific origin to allowed origin list
                    // .allowed_origin("http://project.local:8080")
                    // allow any port on localhost
                    //.allowed_origin_fn(|origin, _req_head| {
                    //    origin.as_bytes().starts_with(b"http://localhost")
                    // })
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(&[header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .expose_headers(&[header::CONTENT_DISPOSITION])
                    // allow cURL/HTTPie from working without providing Origin headers
                    .block_on_origin_mismatch(false)
                    // set preflight cache TTL
                    .max_age(3600),
            )
            .wrap(Compress::default())
            .configure(route)
    };

    // HttpServer::new(app)
    //     .keep_alive(Duration::from_secs(60))
    //     .bind(address)?
    //     .run()
    //     .await

    let server = HttpServer::new(app).keep_alive(Duration::from_secs(60)).bind(address)?.run();

    Ok(server)
}

#[allow(dead_code)]
pub fn run_with_listener(listener: TcpListener, pool: PgPool) -> io::Result<Server> {
    let app_data = web::Data::new(AppState::new(pool));

    let app = move || {
        App::new()
            .app_data(app_data.clone())
            .wrap(ErrorHandlers::new().handler(StatusCode::NOT_FOUND, no_route_error))
            .wrap(
                Cors::default()
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(&[header::AUTHORIZATION, header::ACCEPT])
                    .allowed_header(header::CONTENT_TYPE)
                    .expose_headers(&[header::CONTENT_DISPOSITION])
                    .block_on_origin_mismatch(false)
                    .max_age(3600),
            )
            .wrap(SimpleLogger {})
            .wrap(Compress::default())
            .configure(route)
    };

    let server = HttpServer::new(app).keep_alive(Duration::from_secs(60)).listen(listener)?.run();

    Ok(server)
}
