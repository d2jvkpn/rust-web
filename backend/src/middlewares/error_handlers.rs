// https://actix.rs/docs/middleware/
use super::Error;
use actix_web::{
    dev::{self, ServiceResponse},
    http::header::{HeaderName, HeaderValue},
    middleware::ErrorHandlerResponse,
    ResponseError,
};

// https://docs.rs/actix-web/latest/actix_web/middleware/struct.ErrorHandlers.html
// https://github.com/actix/actix-web/discussions/2564
pub fn no_route_error<B>(sr: ServiceResponse<B>) -> actix_web::Result<ErrorHandlerResponse<B>> {
    // http::header::CONTENT_TYPE, http::header::HeaderValue
    // sr.response_mut().headers_mut().insert(CONTENT_TYPE, HeaderValue::from_static("Error"));
    // Ok(ErrorHandlerResponse::Response(sr.map_into_left_body()))
    if sr.headers().get("content-type").is_some() {
        return Ok(ErrorHandlerResponse::Response(sr.map_into_left_body()));
    }

    // let (req, mut res) = sr.into_parts(); // (HttpRequest, HttpResponse<B>)
    // res.headers_mut().insert(header::CONTENT_TYPE, HeaderValue::from_static("application/json"));
    // let res = res.set_body(r#"{"code":-1,"msg":"no route"}"#.to_owned());
    // let sr = ServiceResponse::new(req, res).map_into_boxed_body().map_into_right_body();
    // Ok(ErrorHandlerResponse::Response(sr))

    let (req, _) = sr.into_parts(); // (HttpRequest, HttpResponse<B>)
    let res = Error::no_route().error_response();
    let sr = ServiceResponse::new(req, res).map_into_boxed_body().map_into_right_body();
    Ok(ErrorHandlerResponse::Response(sr))
}

#[allow(dead_code)]
pub fn add_error_header<B>(
    mut res: dev::ServiceResponse<B>,
) -> actix_web::Result<ErrorHandlerResponse<B>> {
    res.response_mut()
        .headers_mut()
        .insert(HeaderName::from_lowercase(b"x-error").unwrap(), HeaderValue::from_static("Error"));

    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}
