use std::{io, net::TcpListener};

pub fn tcp_listener_with_random_port(ip: &str) -> io::Result<(TcpListener, u16)> {
    let listener = TcpListener::bind(ip)?;
    let port = listener.local_addr()?.port();

    Ok((listener, port))
}
