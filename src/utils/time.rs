#![allow(dead_code)]

use chrono::{Local, SecondsFormat};

fn now() -> String {
    format!(
        "{}",
        Local::now().to_rfc3339_opts(SecondsFormat::Millis, true)
    )
}
