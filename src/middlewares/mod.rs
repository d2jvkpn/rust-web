#![allow(dead_code)]

#[macro_export]
macro_rules! loc2 {
    () => {{
        fn f() {}
        fn type_name_of<T>(_: T) -> &'static str {
            std::any::type_name::<T>()
        }

        // let caller = std::panic::Location::caller();
        let name = type_name_of(f);
        let list: Vec<&str> = name.split("::").collect();
        println!("??? {:?}", list);
        let length = list.len();
        let idx = if list[length - 2] == "{{closure}}" { length - 3 } else { length - 2 };

        format!("{}:{}:{}", file!(), line!(), list[idx])
    }};
}

mod error_handlers;
mod errors;
mod health_check;
mod http_code;
mod into_result;
mod logger;
mod query;
mod record;

pub mod blocker;

pub use error_handlers::*;
pub use errors::*;
pub use health_check::*;
pub use into_result::*;
pub use logger::*;
pub use query::*;
