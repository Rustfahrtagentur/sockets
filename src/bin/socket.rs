use std::io::prelude::*;
use std::net::TcpStream;
//use std::net::{IpAddr, Shutdown, SocketAddr};

fn main() -> std::io::Result<()> {
    let mut stream = TcpStream::connect("127.0.0.1:8080")?;
    let payload = "abc";

    stream.write(&payload.as_bytes())?;
    let mut buf = [0; 128];
    loop {
        stream.read(&mut buf)?;
        stream.write(&mut buf)?;
        let response = std::str::from_utf8(&buf).unwrap();
        println!("{response}");
    }

    //Ok(())
    //let IpAddr(new
    //SocketAddr::new(ip, port
}
