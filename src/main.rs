// Uncomment this block to pass the first stage
use std::{
    io::{Write, Read},
    net::TcpListener,
};

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                let mut resquest = [0; 1024];
                let end = stream.read(&mut resquest).unwrap();
                let parsed = std::str::from_utf8(&resquest[0..end]).unwrap();
                let header = parsed.split("\r\n").nth(0).unwrap();
                let path = header.split(" ").nth(1).unwrap();
                println!("{:?}", path);
                if path == "/" {
                    stream
                        .write_all("HTTP/1.1 200 OK\r\n\r\n".as_bytes())
                        .unwrap();
                } else {
                    stream
                        .write_all("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())
                        .unwrap();
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
