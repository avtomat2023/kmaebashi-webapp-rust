// Section 1.3

use std::error;
use std::net::TcpListener;
use std::io::prelude::*;

type DynResult<T> = std::result::Result<T, Box<dyn error::Error>>;

fn main() -> DynResult<()> {
    {
        let listener = TcpListener::bind("0.0.0.0:8001")?;
        println!("Waiting connection from a client on 127.0.0.1:8001 ...");
        let (mut socket, _addr) = listener.accept()?;
        println!("Connected.");

        for byte in socket.try_clone()?.bytes() {
            let byte = byte?;
            if byte == 0 {
                break;
            }
            println!("{}", byte as char);
        }

        socket.write(b"server echo")?;
    }

    println!("Closed connection.");
    Ok(())
}
