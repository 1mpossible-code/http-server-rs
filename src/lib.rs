use std::{
    fmt::format,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

struct Route {
    path: String,
    body: String,
}

pub struct Controller {
    routes: Vec<Route>,
}

impl Route {
    pub fn new(path: String, body: String) -> Route {
        Route { path, body }
    }
}

impl Controller {
    pub fn new() -> Controller {
        Controller { routes: Vec::new() }
    }

    pub fn add_route(&mut self, path: &str, body: &str) {
        self.routes
            .push(Route::new(path.to_string(), body.to_string()));
    }

    pub fn handle_connection(&self, mut stream: TcpStream) {
        let reader = BufReader::new(&stream);
        let mut lines = reader.lines().next().unwrap().unwrap();
        let path = format!("{}", lines.split_whitespace().nth(1).unwrap());

        println!("path: {}", path);

        let response = "HTTP/1.1 200 OK\r\n\r\n";

        stream.write(response.as_bytes()).unwrap();
    }
}