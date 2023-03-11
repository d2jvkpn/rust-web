macro_rules! loc {
    () => {{
        let caller = std::panic::Location::caller();
        format!("{}:{}", caller.file(), caller.line())
    }};
}

use super::{response::Response};
use actix_web::{
    http::StatusCode, HttpMessage, HttpRequest, HttpResponse,
};
use serde::Serialize;
use serde_json::json;
use uuid::Uuid;

#[derive(Serialize, Debug)]
pub struct Data<T> {
    data: Option<T>,
    #[serde(flatten)]
    response: Response,
}

impl<T: Serialize> Data<T> {
    #[track_caller]
    pub fn new(data: T) -> Self {
        let response = Response {
            code: 0,
            msg: Some("ok".to_string()),
            request_id: Uuid::new_v4(),
            status_code: StatusCode::OK,
            cause: None,
            loc: Some(loc!()),
        };

        Self { data: Some(data), response }
    }

    pub fn into_req(mut self, req: &mut HttpRequest) -> HttpResponse {
        let data = self.data.take();
        let request_id = self.response.request_id;
        req.extensions_mut().insert(self.response);
        HttpResponse::Ok()
            .json(json!({"code": 0,"msg":"ok", "requestId": request_id, "data": data}))
    }
}

impl<T: Serialize> From<T> for Data<T> {
    fn from(d: T) -> Self {
        Self::new(d)
    }
}
