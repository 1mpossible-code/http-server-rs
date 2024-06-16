use std::net::TcpListener;

use http_server_rs::Controller;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    let mut controller = Controller::new();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        controller.handle_connection(stream);
    }
}