use std::io::{Read, Write};
use std::net::TcpListener;
use std::str;
use std::thread;
use std::time::Duration;
fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();
    println!("Listening on port 3000");
    for stream in listener.incoming() {
        let mut _stream = stream.unwrap();
        println!("connection established");
        let mut buff:[u8; 1024] = [0; 1024];
        _stream.read(&mut buff).unwrap();
        println!("{}", str::from_utf8(&mut buff).unwrap());
        thread::sleep(Duration::from_secs(2));
        _stream.write("hello".as_bytes()).unwrap();
        
    }
}
