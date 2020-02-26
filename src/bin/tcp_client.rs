// Section 1.3

use std::error;
use std::net::TcpStream;
use std::io::prelude::*;

type DynResult<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> DynResult<()> {
    let mut socket = TcpStream::connect("127.0.0.1:8001")?;

    socket.write(b"Hello\0")?;

    let mut buf = Vec::new();
    socket.read_to_end(&mut buf)?;
    println!("{}", String::from_utf8_lossy(&buf));

    Ok(())
}
