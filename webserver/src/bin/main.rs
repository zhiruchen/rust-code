use std::fs;
use std::io::prelude::*;
use std::net::{ TcpListener, TcpStream};

use webserver::ThreadPool;

fn main() {
    let lisener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in lisener.incoming(){
        let stream = stream.unwrap();

        pool::execute(|| {
            handle_connection(stream);
        });
        
        println!("Connection is established");
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";

    let (status_line, file_name) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "page.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let contents = fs::read_to_string(file_name).unwrap();

    let resp = format!("{}{}", status_line, contents);
    stream.write(resp.as_bytes()).unwrap();
    stream.flush().unwrap();
}
