use std::{
    collections::HashMap,
    io::{BufRead, BufReader, Write},
    net::TcpStream,
};

pub struct Controller {
    routes: HashMap<String, String>,
}

impl Controller {
    pub fn new() -> Controller {
        Controller {
            routes: HashMap::new(),
        }
    }

    pub fn add_route(&mut self, path: &str, body: &str) {
        self.routes.insert(path.to_string(), body.to_string());
    }

    pub fn handle_connection(&self, mut stream: TcpStream) {
        let reader = BufReader::new(&stream);
        let lines = reader.lines().next().unwrap().unwrap();
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
        if self.routes.contains_key(path) {
            let route = self.routes.get(path).unwrap();
            if route.ends_with(".html") {
                let body = std::fs::read_to_string(&route).unwrap();
                return body;
            }
            return route.clone();
        }

        String::from("404 Not Found")
    }
}
