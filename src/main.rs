mod resource_handling;
use crate::resource_handling::handling::*;
use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

pub struct Resource<'a> {
    url: &'a str,
    path: &'a str,
}

fn main() {
    set_up_server();
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                handle_connection(stream);
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
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();
    let first_line: Vec<String> = http_request[0]
        .trim()
        .split(" ")
        .map(|val| val.parse().unwrap())
        .collect();

    let (http_method, resource) = (&first_line[0], &first_line[1]);

    let contents = if (resource.eq("/")) {
        include_str!("../resources/hello.html").to_string()
    } else {
        // TODO: do something with the rest of the files like how do I handle pics and js?
        include_str!("../resources/hello.html").to_string()
        // println!("{}", resource);
        // let path = "./resources".to_owned() + resource;
        // fs::read_to_string(path).unwrap().to_string()
    };
    println!("httpMethod: {}, resources: {}", http_method, resource);
    let status_line = "HTTP/1.1 200 OK";
    let length = contents.len();
    //TODO: set all the other headers here as well?
    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");
    println!("Request: {:#?}", http_request);
    println!("{}", response);
    stream.write_all(response.as_bytes()).unwrap()
}
