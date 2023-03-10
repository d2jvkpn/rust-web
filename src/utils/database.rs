use sqlx::{error::Error as SQLxError, types::ipnetwork::IpNetwork};
use std::net::{IpAddr, SocketAddr};

// TODO: sqlx::error::Error, sqlx::postgres::PgDatabaseError,
pub fn db_error_code(err: &SQLxError) -> Option<String> {
    let e2 = match err {
        SQLxError::Database(e) => e,
        _ => return None,
    };

    e2.code().map(|v| Some(v.into()))? // convert a Result to an option
}

pub fn pg_already_exists(err: &SQLxError) -> bool {
    match db_error_code(err) {
        None => false,
        Some(ref v) => v == "23505",
    }
}

pub fn pg_not_found(err: &SQLxError) -> bool {
    matches!(err, SQLxError::RowNotFound)
}

pub fn socket_addr_to_ip_network(socket_addr: &SocketAddr) -> Option<IpNetwork> {
    let ip = socket_addr.ip();

    let prefix = match ip {
        IpAddr::V4(_) => 32,
        IpAddr::V6(_) => 128,
    };

    IpNetwork::new(ip, prefix).ok() // ?? use expect
}
