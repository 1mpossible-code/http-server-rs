use http_server_rs::Controller;
use std::net::TcpListener;
use std::thread;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            let mut controller = Controller::new();

            controller.add_route("/test", "test.html");

            controller.handle_connection(stream);
        });
    }
}
