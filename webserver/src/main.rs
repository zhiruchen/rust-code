use std::io::prelude::*;
use std::net::{ TcpListener, TcpStream};

fn main() {
    let lisener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in lisener.incoming(){
        let stream = stream.unwrap();
        handle_connection(stream);
        println!("Connection is established");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));
}
