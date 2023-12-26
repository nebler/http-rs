use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};
mod resource_handler;
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    resource_handler::setUpServer();
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
                println!("accepted new connection");
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    stream.write("HTTP/1.1 200 OK\r\n\r\n".as_bytes()).unwrap();
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let first_line: Vec<String> = http_request[0]
        .trim()
        .split(" ")
        .map(|val| val.parse().unwrap())
        .collect();

    let (http_method, resource) = (&first_line[0], &first_line[1]);
    println!("httpMethod: {}, resources: {}", http_method, resource);
    let status_line = "HTTP/1.1 200 OK";
    let contents = include_str!("./resources/hello.html");
    let length = contents.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    println!("Request: {:#?}", http_request);
    println!("{}", response);
    stream.write_all(contents.as_bytes()).unwrap();
}
