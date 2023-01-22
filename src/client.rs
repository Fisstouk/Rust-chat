use std::io::prelude::*;
use std::net::TcpStream;

pub fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:5000").unwrap();

    stream.write(&[1])?;
    stream.read(&mut [0; 128])?;
    Ok(())
} // the stream is closed here