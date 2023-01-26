use std::net::{SocketAddr, TcpListener};

pub fn main() {

    let listener = TcpListener::bind("127.0.0.1:5000").expect("Socket not bound to local host");

    let mut server_socket: Vec<SocketAddr> = loop {
        match listener.accept() {
            Ok((_socket, addr)) => {return;}, // Break early and return addr
            Err(e) => println!("Couldn't get client: {e:?}"),
        };
    };
    assert_eq!("127.0.0.1:5000".parse().unwrap(), server_socket);
    // Keep socket thus keep connection with client
    server_socket.push(server_socket);
}