
use std:: {
    io::{Read, Write},
    net::TcpStream,
    str,
};

fn main() {
    let mut _stream = TcpStream::connect("127.0.0.1:3000").unwrap();
    _stream.write("he".as_bytes()).unwrap();
    let mut buf:[u8; 5] = [0; 5];
    _stream.read(&mut buf).unwrap();
    println!("{}", str::from_utf8(&mut buf).unwrap());
}
