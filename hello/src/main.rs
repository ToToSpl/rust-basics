use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();

    let (filename, status_line) = if request_line == "GET / HTTP/1.1" {
        ("hello.html", "HTTP/1.1 200 OK")
    } else {
        ("404.html", "HTTP/1.1 404 NOT FOUND")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let lenght = contents.len();

    let response = format!("{status_line}\r\nContent-Lenght: {lenght}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
}
