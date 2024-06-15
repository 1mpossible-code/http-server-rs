use std::net::{TcpListener, TcpStream};
use std::io::{BufRead, BufReader};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream)
    }
}

fn handle_connection(stream: TcpStream) {
    let reader = BufReader::new(stream);
    let mut lines = reader.lines();

    while let Some(line) = lines.next() {
        let line = line.unwrap();
        println!("{}", line);
    }
}