use std::net::TcpListener;

pub fn main ()
{
let listener = TcpListener::bind("127.0.0.1:5000").expect("Socket not bound to local host");

match listener.accept() 
    {
        Ok((_socket, addr)) => println!("New client: {addr:?}"),
        Err(e) => println!("Couldn't get client: {e:?}"),
    }

}