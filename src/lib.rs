use std::{io::{BufRead, BufReader, Write}, net::TcpStream};


pub fn handle_connection(mut stream: TcpStream) {
    let reader = BufReader::new(&stream);
    let mut lines = reader.lines().map(|l| l.unwrap()).take_while(|l| l != "");

    while let Some(line) = lines.next() {
        println!("{}", line);
    }

    let response = "HTTP/1.1 200 OK\r\n\r\n";

    // Send html file as response
    let html = include_str!("test.html");
    let response = format!("{}{}", response, html);


    stream.write(response.as_bytes()).unwrap();
}