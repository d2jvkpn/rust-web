use actix_web::{HttpMessage, HttpRequest, HttpResponse};
use serde::Serialize;
use serde_json::json;
use uuid::Uuid;

#[derive(Debug, Serialize)]
pub struct Data<T>(pub T);

impl<T: Serialize> Data<T> {
    pub fn into_req(self, req: &mut HttpRequest) -> HttpResponse {
        let request_id = Uuid::new_v4();
        req.extensions_mut().insert(request_id);
        HttpResponse::Ok()
            .json(json!({"code": 0, "msg":"ok", "requestId": request_id, "data": self.0}))
    }
}

impl<T: Serialize> From<T> for Data<T> {
    fn from(d: T) -> Self {
        Data(d)
    }
}
