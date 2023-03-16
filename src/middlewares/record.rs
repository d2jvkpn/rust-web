// https://actix.rs/docs/middleware/
use super::errors::Error as AnError;
use actix_web::HttpRequest;
use chrono::{DateTime, Local};
use log::{error, info, warn};
use serde::Serialize;
// use serde_json::json;
use std::fmt;
use uuid::Uuid;

#[derive(Debug, Serialize, Default)]
pub struct Record {
    #[serde(skip)]
    pub start_at: DateTime<Local>,

    pub request_id: Uuid,
    pub method: String,
    pub path: String,
    pub status: u16,
    pub user_id: Option<i32>,
    pub elapsed: String,
    pub code: i32,
    pub msg: String,
    pub cause: Option<String>,
    pub loc: Option<String>,
}

impl fmt::Display for Record {
    fn fmt(&self, w: &mut fmt::Formatter) -> fmt::Result {
        write!(
            w,
            "request_id: {}, method: {}, path: {:?}, status: {}, user_id: {:?}, \
            elapsed: {}, code: {}, msg: {:?}, cause: {:?}, loc: {:?}",
            self.request_id,
            self.method,
            self.path,
            self.status,
            self.user_id,
            self.elapsed,
            self.code,
            self.msg,
            self.cause,
            self.loc
        )
    }
}

impl Record {
    pub fn from_request(req: &HttpRequest) -> Self {
        Record {
            start_at: Local::now(),
            request_id: Uuid::new_v4(),
            method: req.method().to_string(),
            path: req.path().to_string(),
            msg: "ok".into(),
            ..Default::default()
        }
    }

    pub fn elapsed(&mut self) {
        let end: DateTime<Local> = Local::now();
        let elapsed = end.signed_duration_since(self.start_at).num_microseconds().unwrap_or(0);
        self.elapsed = format!("{:.3}ms", (elapsed as f64) / 1e3);
    }

    // consume a response::Response, using Option<T>.take() rather than Option<T>.clone()
    pub fn with_error(&mut self, mut err: AnError) {
        self.code = err.code;
        self.msg = err.msg;
        self.status = err.status.as_u16();
        if let Some(e) = err.cause {
            self.cause = Some(format!("{:}", e));
        }
        self.loc = err.loc.take();
    }

    pub fn log(&self) {
        if self.status >= 500 {
            // error!("{}", json!(self));
            error!("{}", self);
        } else if self.status >= 400 {
            // warn!("{}", json!(self));
            warn!("{}", self);
        } else {
            // info!("{}", json!(self));
            info!("{}", self);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn t_record() {
        let mut record = Record::default();
        record.msg = Some("Hello".into());
        println!("{}", record);
    }
}
