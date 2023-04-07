use chrono::{Local, SecondsFormat};

fn now() -> String {
    Local::now().to_rfc3339_opts(SecondsFormat::Millis, true)
}
