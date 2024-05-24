// Uncomment this block to pass the first stage
use std::net::TcpListener;
use std::io::{prelude::*, BufReader};
use std::net::TcpStream;

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                handle_connection(_stream);
                // _stream.write(b"HTTP/1.1 200 OK\r\n\r\n").expect("200 \n");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|line| line.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("{:#?}", http_request);

    let start_line = http_request.first().unwrap();
    let path = start_line.split_whitespace().nth(1).unwrap();
    let response: &str;
    if path == "/" {
        response = "HTTP/1.1 200 OK\r\n\r\n";
    } else {
        response = "HTTP/1.1 404 Not Found\r\n\r\n";
    }

    stream.write_all(response.as_bytes()).unwrap();
}
