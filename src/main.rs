use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader, Write};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream)
    }
}

fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&stream);
    let mut lines = reader.lines().map(|l| l.unwrap()).take_while(|l| l != "");

    while let Some(line) = lines.next() {
        println!("{}", line);
    }

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    stream.write(response.as_bytes()).unwrap();
}