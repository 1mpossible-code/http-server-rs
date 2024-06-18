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
        let path = Controller::get_path(&lines);

        let response = "HTTP/1.1 200 OK\r\n\r\n";

        let content_body = self.get_content_body(&path);

        let response = format!("{}{}", response, content_body);

        stream.write(response.as_bytes()).unwrap();
    }

    fn get_path(header_line: &str) -> String {
        let path = header_line.split_whitespace().nth(1).unwrap();
        path.to_string()
    }

    fn get_content_body(&self, path: &str) -> String {
        for route in &self.routes {
            if route.path == path {
                if route.body.ends_with(".html") {
                    let body = std::fs::read_to_string(&route.body).unwrap();
                    return body;
                }
                return route.body.clone();
            }
        }

        String::from("404 Not Found")
    }
}